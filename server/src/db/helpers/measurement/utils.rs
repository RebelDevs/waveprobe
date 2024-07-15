use crate::db::models::measurement::{Measurement, MeasurementRow, OptionsEnum, Status};

pub fn row_to_measurement(row: MeasurementRow) -> Measurement {
    let command = row.command.clone();
    let status = match row.status.to_string().as_str() {
        "pending" => Status::Pending,
        "processing" => Status::Processing,
        "completed" => Status::Completed,
        "error" => Status::Error,
        &_ => Status::Error,
    };

    let options = match command.as_str() {
        "ping" => serde_json::from_value(row.parameters)
            .map(OptionsEnum::Ping)
            .unwrap(),
        &_ => todo!(),
    };

    return Measurement {
        id: row.id,
        status,
        command: row.command,
        parameters: Some(options),
        location: row.location,
        created_at: row.created_at,
        updated_at: row.updated_at,
    };
}
