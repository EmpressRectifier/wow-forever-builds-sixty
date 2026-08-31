//! Plan the zone stops for a build: where the route takes you.

pub struct ZoneStop {
    pub level: u8,
    pub zone: &'static str,
}

const ZONES: &[ZoneStop] = &[
    ZoneStop { level: 1,  zone: "Elwynn Forest" },
    ZoneStop { level: 10, zone: "Westfall" },
    ZoneStop { level: 18, zone: "Redridge Mountains" },
    ZoneStop { level: 24, zone: "Duskwood" },
    ZoneStop { level: 30, zone: "Stranglethorn Vale" },
    ZoneStop { level: 38, zone: "Badlands" },
    ZoneStop { level: 45, zone: "Searing Gorge" },
    ZoneStop { level: 50, zone: "Un'Goro Crater" },
    ZoneStop { level: 55, zone: "Western Plaguelands" },
];

pub fn stops_for(level: u8) -> impl Iterator<Item = &'static ZoneStop> {
    ZONES.iter().take_while(move |z| z.level <= level + 4)
}
