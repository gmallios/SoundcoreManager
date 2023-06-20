use uuid::Uuid;

use crate::bt::ble::BLEConnectionUuidSet;

pub mod a3027;
pub mod a3951;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SupportedModelIDs {
    A3951,
    A3027,
}

pub fn match_name_to_model_id(name: &str) -> Option<SupportedModelIDs> {
    let models = vec![
        ("Soundcore Liberty Air 2 Pro", SupportedModelIDs::A3951),
        ("Soundcore Life Q35", SupportedModelIDs::A3027),
    ];

    for (model_name, model_id) in models {
        if name.contains(model_name) {
            return Some(model_id);
        }
    }

    None
}

pub fn match_model_id_to_uuid_set(model_id: &SupportedModelIDs) -> Option<BLEConnectionUuidSet> {
    match model_id {
        SupportedModelIDs::A3951 => Some(BLEConnectionUuidSet {
            service_uuid: Uuid::parse_str("011AF5DA-0000-1000-8000-00805F9B34FB").unwrap(),
            write_uuid: Uuid::parse_str("00007777-0000-1000-8000-00805f9b34fb").unwrap(),
            read_uuid: Uuid::parse_str("00008888-0000-1000-8000-00805F9B34FB").unwrap(),
        }),
        // SupportedModelIDs::A3027 => Some(BLEConnectionUuidSet {
        //     service_uuid: Uuid::parse_str("0000fe95-0000-1000-8000-00805f9b34fb").unwrap(),
        //     write_uuid: Uuid::parse_str("8b00ace7-eb0b-49b0-bbe9-9aee0a26e1a3").unwrap(),
        //     read_uuid: Uuid::parse_str("0734594a-a8e7-4b1a-a6b1-cd5243059a57").unwrap(),
        // }),
        _ => None,
    }
}
