use std::{fs::File, io::BufReader};

use color_eyre::eyre::Result;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

pub struct AudioPlayer {
    stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let stream =
            OutputStreamBuilder::open_default_stream().expect("Error al inicializar las bocinas");
        Self { stream }
    }

    pub fn play(&self, file: File) -> Result<()> {
        let sink = Sink::connect_new(self.stream.mixer());

        // Abre el archivo mp3
        let source = Decoder::new(BufReader::new(file))?;

        // Lo mandas al sink
        sink.append(source);

        // Esperas a que termine de reproducir
        sink.sleep_until_end();

        Ok(())
    }
}
