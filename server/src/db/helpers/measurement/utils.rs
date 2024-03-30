use crate::db::models::measurement::{Measurement, MeasurementRow, OptionsEnum};

pub fn row_to_measurement(row: MeasurementRow) -> Measurement {
    let command = row.command.clone();
    let options = match command.as_str() {
        "ping" => serde_json::from_value(row.parameters)
            .map(OptionsEnum::Ping)
            .unwrap(),
        &_ => todo!(),
    };

    return Measurement {
        id: row.id,
        command: row.command,
        parameters: Some(options),
        location: row.location,
        created_at: row.created_at,
        updated_at: row.updated_at,
    };
}
