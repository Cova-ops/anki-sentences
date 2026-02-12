use std::path::PathBuf;

use crate::{db::traits::SqlNew, services::tts::eleven_labs::EnumVoiceIDElevenLabs};

#[derive(Debug)]
pub struct SqlSetzeAudio {
    pub satz_id: i32,
    pub file_path: String,
    pub voice_id: String,
}

#[derive(Debug, Clone)]
pub struct InputSetzeAudio {
    pub satz_id: i32,
    pub file_path: PathBuf,
    pub voice: EnumVoiceIDElevenLabs,
}

impl From<InputSetzeAudio> for SqlSetzeAudio {
    fn from(value: InputSetzeAudio) -> Self {
        Self {
            satz_id: value.satz_id,
            file_path: value.file_path.to_string_lossy().into_owned(),
            voice_id: value.voice.get_key().to_string(),
        }
    }
}

impl SqlNew for SqlSetzeAudio {
    type Params<'a>
        = (
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
    )
    where
        Self: 'a;

    /// This orden:
    /// - satz_id
    /// - file_path
    /// - voice_id
    fn to_params<'a>(&'a self) -> Self::Params<'a> {
        (&self.satz_id, &self.file_path, &self.voice_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_input {
        use super::*;

        #[test]
        fn input_setze_audio_into_sql_setze_audio_maps_fields() {
            let voice = EnumVoiceIDElevenLabs::GermanMan;

            let input = InputSetzeAudio {
                satz_id: 42,
                file_path: PathBuf::from("audios/setze/satz_000042_de.mp3"),
                voice,
            };

            let sql: SqlSetzeAudio = input.into();

            assert_eq!(sql.satz_id, 42);
            assert_eq!(sql.file_path, "audios/setze/satz_000042_de.mp3");
            assert_eq!(sql.voice_id, voice.get_key());
        }

        #[test]
        fn input_setze_audio_path_is_string_lossy() {
            let voice = EnumVoiceIDElevenLabs::SpanishWoman;

            let input = InputSetzeAudio {
                satz_id: 1,
                file_path: PathBuf::from("/tmp/áéíóú.mp3"),
                voice,
            };

            let sql: SqlSetzeAudio = input.into();

            assert_eq!(sql.satz_id, 1);
            assert_eq!(sql.file_path, "/tmp/áéíóú.mp3");
            assert_eq!(sql.voice_id, voice.get_key());
        }
    }

    mod sql_new {
        use super::*;

        use rusqlite::ToSql;
        use rusqlite::types::{ToSqlOutput, Value, ValueRef};

        fn to_value(p: &dyn ToSql) -> Value {
            match p.to_sql().expect("to_sql should work") {
                ToSqlOutput::Owned(v) => v,
                ToSqlOutput::Borrowed(vr) => match vr {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(i) => Value::Integer(i),
                    ValueRef::Real(f) => Value::Real(f),
                    ValueRef::Text(t) => Value::Text(String::from_utf8_lossy(t).into_owned()),
                    ValueRef::Blob(b) => Value::Blob(b.to_vec()),
                },
                _ => panic!(""),
            }
        }

        #[test]
        fn to_params_returns_values_in_expected_order() {
            let s = SqlSetzeAudio {
                satz_id: 1,
                file_path: String::from("tmp"),
                voice_id: EnumVoiceIDElevenLabs::GermanMan.get_key().to_string(),
            };

            let (p1, p2, p3) = s.to_params();

            assert_eq!(to_value(p1), Value::Integer(1));
            assert_eq!(to_value(p2), Value::Text(String::from("tmp")));
            assert_eq!(
                to_value(p3),
                Value::Text(EnumVoiceIDElevenLabs::GermanMan.get_key().to_string())
            );
        }
    }
}
