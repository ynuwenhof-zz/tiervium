use crate::models::{Log, Vehicle};
use sqlx::MySqlPool;

pub async fn add_vehicles(pool: &MySqlPool, vehicles: &[Vehicle]) -> anyhow::Result<()> {
    if vehicles.is_empty() {
        return Ok(());
    }

    let values: Vec<String> = vehicles
        .iter()
        .map(|v| {
            format!(
                "('{}', {}, {}, {}, {}, '{}', '{}', '{}', '{}')",
                v.uuid,
                v.code,
                v.max_speed,
                v.has_box,
                v.has_helmet,
                v.zone,
                v.kind,
                v.vendor,
                v.license_plate
            )
        })
        .collect();

    let query = format!(
        r#"
INSERT INTO vehicles (uuid, code, max_speed, has_box, has_helmet, zone, kind, vendor, license_plate)
VALUES {} ON DUPLICATE KEY UPDATE uuid = uuid
        "#,
        values.join(",")
    );

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn add_logs(pool: &MySqlPool, logs: &[Log]) -> anyhow::Result<()> {
    if logs.is_empty() {
        return Ok(());
    }

    let values: Vec<String> = logs
        .iter()
        .map(|l| {
            format!(
                "('{}', '{}', {}, {}, {}, {}, '{}')",
                l.vehicle_uuid,
                l.time.format("%Y-%m-%d %H:%M:%S"),
                l.lat,
                l.lng,
                l.battery,
                l.rentable,
                l.state
            )
        })
        .collect();

    let query = format!(
        r#"
INSERT INTO logs (vehicle_uuid, time, lat, lng, battery, rentable, state)
VALUES {}
        "#,
        values.join(",")
    );

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn latest_log(
    pool: &MySqlPool,
    vehicle_uuid: impl AsRef<str>,
) -> anyhow::Result<Option<Log>> {
    Ok(sqlx::query_as::<_, Log>(
        r#"
SELECT vehicle_uuid, MAX(time) AS time, lat, lng, battery, rentable, state
FROM logs
WHERE vehicle_uuid = ?
HAVING time IS NOT NULL
LIMIT 1
            "#,
    )
    .bind(vehicle_uuid.as_ref())
    .fetch_optional(pool)
    .await?)
}
