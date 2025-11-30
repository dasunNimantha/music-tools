use anyhow::Result;
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

pub fn play_streaming_url_async(url: String) {
    // Stop any currently playing audio
    stop_audio();

    // Use a detached std::thread that won't prevent app shutdown
    // The thread will be terminated when the process exits
    let _handle = std::thread::spawn(move || {
        // Create a minimal tokio runtime just for this thread to do HTTP requests
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("Failed to create runtime for audio: {}", e);
                return;
            }
        };

        let response = match rt.block_on(reqwest::get(&url)) {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Failed to fetch audio URL: {}", e);
                return;
            }
        };

        if !response.status().is_success() {
            eprintln!("Failed to fetch audio: HTTP {}", response.status());
            return;
        }

        let bytes = match rt.block_on(response.bytes()) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Failed to read audio data: {}", e);
                return;
            }
        };

        // Now do the audio playback (blocking operation)
        {
            let (_stream, stream_handle) = match OutputStream::try_default() {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to create audio output stream: {}", e);
                    return;
                }
            };

            let sink = match Sink::try_new(&stream_handle) {
                Ok(sink) => sink,
                Err(e) => {
                    eprintln!("Failed to create audio sink: {}", e);
                    return;
                }
            };

            // Store the sink so we can stop it
            let sink_arc = Arc::new(Mutex::new(sink));
            if let Ok(mut sink_guard) = AUDIO_SINK.lock() {
                *sink_guard = Some(sink_arc.clone());
            }

            let cursor = Cursor::new(bytes.to_vec());
            let source = match Decoder::new(cursor) {
                Ok(source) => source,
                Err(e) => {
                    eprintln!("Failed to decode audio: {}", e);
                    if let Ok(mut sink_guard) = AUDIO_SINK.lock() {
                        sink_guard.take();
                    }
                    return;
                }
            };

            // Append source to sink
            {
                let sink = sink_arc.lock().unwrap();
                sink.append(source);
            }

            // Wait for playback to finish with shorter sleep intervals for responsiveness
            loop {
                std::thread::sleep(std::time::Duration::from_millis(50));

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
        }
    });
    // Handle is dropped here, making the thread detached
    // The thread will continue running but won't prevent process exit
}

// Keep the old async function for compatibility, but make it non-blocking
pub async fn play_streaming_url(url: String) -> Result<()> {
    play_streaming_url_async(url);
    Ok(())
}
