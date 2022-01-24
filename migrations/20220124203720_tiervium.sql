CREATE TABLE IF NOT EXISTS vehicles (
    uuid char(36) NOT NULL,
    code int NOT NULL,
    max_speed int NOT NULL,
    has_box boolean NOT NULL,
    has_helmet boolean NOT NULL,
    zone varchar(45) NOT NULL,
    kind varchar(45) NOT NULL,
    vendor varchar(45) NOT NULL,
    license_plate varchar(6) NOT NULL,
    PRIMARY KEY (uuid)
);

CREATE TABLE IF NOT EXISTS logs (
    id int NOT NULL AUTO_INCREMENT,
    vehicle_uuid char(36) NOT NULL,
    time datetime NOT NULL,
    lat float NOT NULL,
    lng float NOT NULL,
    battery int NOT NULL,
    rentable boolean NOT NULL,
    state varchar(45) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (vehicle_uuid) REFERENCES vehicles(uuid)
);
