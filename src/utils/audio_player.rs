use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

static AUDIO_SINK: Mutex<Option<Arc<Mutex<Sink>>>> = Mutex::new(None);

pub fn stop_audio() {
    if let Ok(mut sink_guard) = AUDIO_SINK.lock() {
        if let Some(sink) = sink_guard.take() {
            if let Ok(sink) = sink.lock() {
                sink.stop();
                sink.clear();
            }
        }
    }
}

pub async fn play_streaming_url(url: String) -> Result<()> {
    // Stop any currently playing audio
    stop_audio();

    let response = reqwest::get(&url)
        .await
        .context("Failed to fetch audio URL")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch audio: HTTP {}", response.status());
    }

    let bytes = response
        .bytes()
        .await
        .context("Failed to read audio data")?;

    tokio::task::spawn_blocking(move || {
        let (_stream, stream_handle) =
            OutputStream::try_default().context("Failed to create audio output stream")?;

        let sink = Sink::try_new(&stream_handle).context("Failed to create audio sink")?;

        // Store the sink so we can stop it
        let sink_arc = Arc::new(Mutex::new(sink));
        if let Ok(mut sink_guard) = AUDIO_SINK.lock() {
            *sink_guard = Some(sink_arc.clone());
        }

        let cursor = Cursor::new(bytes.to_vec());
        let source = Decoder::new(cursor).context("Failed to decode audio")?;

        // Append source to sink
        {
            let sink = sink_arc.lock().unwrap();
            sink.append(source);
        }

        // Wait for playback to finish
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Check if playback finished or was stopped
            let sink = sink_arc.lock().unwrap();
            if sink.empty() {
                drop(sink);
                break;
            }
            drop(sink);

            // Check if sink was removed (stopped)
            if let Ok(sink_guard) = AUDIO_SINK.lock() {
                if sink_guard.is_none() {
                    break;
                }
            }
        }

        // Clean up
        if let Ok(mut sink_guard) = AUDIO_SINK.lock() {
            sink_guard.take();
        }

        Ok(())
    })
    .await
    .context("Audio playback task failed")?
}
