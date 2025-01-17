use api::data::station_history::StationHistoryCurrent;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct StationCachedInfo {
    ok: bool,
    message: String,
    id: i32,
    stationuuid: String,
    name: String,
    url: String,
}

impl StationCachedInfo {
    pub fn serialize_cached_info(station: StationCachedInfo) -> std::io::Result<String> {
        let mut xml = xml_writer::XmlWriter::new(Vec::new());
        xml.begin_elem("result")?;
        xml.begin_elem("status")?;
        xml.attr_esc("ok", &station.ok.to_string())?;
        xml.attr_esc("message", &station.message)?;
        xml.attr_esc("id", &station.id.to_string())?;
        xml.attr_esc("stationuuid", &station.stationuuid)?;
        xml.attr_esc("name", &station.name)?;
        xml.attr_esc("url", &station.url)?;
        xml.end_elem()?;
        xml.end_elem()?;
        xml.close()?;
        xml.flush()?;
        Ok(String::from_utf8(xml.into_inner()).unwrap_or("encoding error".to_string()))
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Station {
    pub id: i32,
    pub changeuuid: String,
    pub stationuuid: String,
    pub name: String,
    pub url: String,
    #[serde(skip_serializing)]
    pub urlcache: String,
    pub homepage: String,
    pub favicon: String,
    pub tags: String,
    pub country: String,
    pub countrycode: String,
    pub state: String,
    pub language: String,
    pub votes: i32,
    pub negativevotes: i32,
    pub lastchangetime: String,
    pub ip: String,
    pub codec: String,
    pub bitrate: u32,
    pub hls: i8,
    pub lastcheckok: i8,
    pub lastchecktime: String,
    pub lastcheckoktime: String,
    pub clicktimestamp: String,
    pub clickcount: u32,
    pub clicktrend: i32,
}

impl Station {
    pub fn new(
        id: i32,
        changeuuid: String,
        stationuuid: String,
        name: String,
        url: String,
        urlcache: String,
        homepage: String,
        favicon: String,
        tags: String,
        country: String,
        countrycode: String,
        state: String,
        language: String,
        votes: i32,
        negativevotes: i32,
        lastchangetime: String,
        ip: String,
        codec: String,
        bitrate: u32,
        hls: i8,
        lastcheckok: i8,
        lastchecktime: String,
        lastcheckoktime: String,
        clicktimestamp: String,
        clickcount: u32,
        clicktrend: i32,
    ) -> Self {
        Station {
            id,
            changeuuid,
            stationuuid,
            name,
            url,
            urlcache,
            homepage,
            favicon,
            tags,
            country,
            countrycode,
            state,
            language,
            votes,
            negativevotes,
            lastchangetime,
            ip,
            codec,
            bitrate,
            hls,
            lastcheckok,
            lastchecktime,
            lastcheckoktime,
            clicktimestamp,
            clickcount,
            clicktrend,
        }
    }

    pub fn extract_cached_info(station: Station, message: &str) -> StationCachedInfo {
        return StationCachedInfo {
            ok: station.lastcheckok == 1,
            message: message.to_string(),
            id: station.id,
            stationuuid: station.stationuuid,
            name: station.name,
            url: station.urlcache,
        };
    }

    pub fn serialize_station_list(entries: Vec<Station>) -> std::io::Result<String> {
        let mut xml = xml_writer::XmlWriter::new(Vec::new());
        xml.begin_elem("result")?;
        for entry in entries {
            xml.begin_elem("station")?;
            let station_id_str = format!("{}", entry.id);
            xml.attr_esc("id", &station_id_str)?;
            xml.attr_esc("changeuuid", &entry.changeuuid)?;
            xml.attr_esc("stationuuid", &entry.stationuuid)?;
            xml.attr_esc("name", &entry.name)?;
            xml.attr_esc("url", &entry.url)?;
            xml.attr_esc("homepage", &entry.homepage)?;
            xml.attr_esc("favicon", &entry.favicon)?;
            xml.attr_esc("tags", &entry.tags)?;
            xml.attr_esc("country", &entry.country)?;
            xml.attr_esc("countrycode", &entry.countrycode)?;
            xml.attr_esc("state", &entry.state)?;
            xml.attr_esc("language", &entry.language)?;
            let station_votes_str = format!("{}", entry.votes);
            xml.attr_esc("votes", &station_votes_str)?;
            let station_negativevotes_str = format!("{}", entry.negativevotes);
            xml.attr_esc("negativevotes", &station_negativevotes_str)?;
            let station_lastchangetime_str = format!("{}", entry.lastchangetime);
            xml.attr_esc("lastchangetime", &station_lastchangetime_str)?;
            xml.attr_esc("ip", &entry.ip)?;
            xml.attr_esc("codec", &entry.codec)?;
            let station_bitrate = format!("{}", entry.bitrate);
            xml.attr_esc("bitrate", &station_bitrate)?;
            let station_hls = format!("{}", entry.hls);
            xml.attr_esc("hls", &station_hls)?;
            let station_lastcheckok = format!("{}", entry.lastcheckok);
            xml.attr_esc("lastcheckok", &station_lastcheckok)?;
            let station_lastchecktime_str = format!("{}", entry.lastchecktime);
            xml.attr_esc("lastchecktime", &station_lastchecktime_str)?;
            let station_lastcheckoktime_str = format!("{}", entry.lastcheckoktime);
            xml.attr_esc("lastcheckoktime", &station_lastcheckoktime_str)?;
            let station_clicktimestamp_str = format!("{}", entry.clicktimestamp);
            xml.attr_esc("clicktimestamp", &station_clicktimestamp_str)?;
            let station_clickcount = format!("{}", entry.clickcount);
            xml.attr_esc("clickcount", &station_clickcount)?;
            let station_clicktrend = format!("{}", entry.clicktrend);
            xml.attr_esc("clicktrend", &station_clicktrend)?;
            xml.end_elem()?;
        }
        xml.end_elem()?;
        xml.close()?;
        xml.flush()?;
        Ok(String::from_utf8(xml.into_inner()).unwrap_or("encoding error".to_string()))
    }

    pub fn serialize_to_m3u(list: Vec<Station>, use_cached_url: bool) -> String {
        let mut j = String::with_capacity(200 * list.len());
        j.push_str("#EXTM3U\r\n");
        for item in list {
            j.push_str("#EXTINF:1,");
            j.push_str(&item.name);
            j.push_str("\r\n");
            if use_cached_url {
                j.push_str(&item.urlcache);
            } else {
                j.push_str(&item.url);
            }
            j.push_str("\r\n\r\n");
        }
        j
    }

    pub fn serialize_to_pls(list: Vec<Station>, use_cached_url: bool) -> String {
        let mut j = String::with_capacity(200 * list.len());
        j.push_str("[playlist]\r\n");
        let mut i = 1;
        for item in list {
            let i_str = i.to_string();
            j.push_str("File");
            j.push_str(&i_str);
            j.push_str("=");
            j.push_str(&item.name);
            j.push_str("\r\n");
            j.push_str("Title");
            j.push_str(&i_str);
            j.push_str("=");
            if use_cached_url {
                j.push_str(&item.urlcache);
            } else {
                j.push_str(&item.url);
            }
            j.push_str("\r\n\r\n");
            i += 1;
        }
        j
    }

    pub fn serialize_to_xspf(entries: Vec<Station>) -> std::io::Result<String> {
        let mut xml = xml_writer::XmlWriter::new(Vec::new());
        xml.dtd("UTF-8")?;
        xml.begin_elem("playlist")?;
        xml.attr_esc("version", "1")?;
        xml.attr_esc("xmlns", "http://xspf.org/ns/0/")?;
        xml.begin_elem("trackList")?;
        for entry in entries {
            xml.begin_elem("track")?;
            xml.elem_text("title", &entry.name)?;
            xml.elem_text("location", &entry.url)?;
            xml.end_elem()?;
        }
        xml.end_elem()?;
        xml.end_elem()?;
        xml.close()?;
        xml.flush()?;
        Ok(String::from_utf8(xml.into_inner()).unwrap_or("encoding error".to_string()))
    }

    // Syntax checked with http://ttl.summerofcode.be/
    fn serialize_to_ttl_single(&self) -> String {
        format!(
            r#"<http://radio-browser.info/radio/{id}>
    rdf:type schema:RadioStation ;
    dcterms:identifier "{id}" ;
    schema:PropertyValue [
        schema:name "changeuuid" ;
        schema:value "{changeuuid}"
    ] ;
    schema:PropertyValue [
        schema:name "stationuuid" ;
        schema:value "{stationuuid}"
    ] ;
    schema:name "{name}" ;
    schema:url <{url}> ;
    schema:sameAs <{homepage}> ;
    schema:logo <{favicon}> ;
    schema:Country [
        schema:name "{country}" ;
    ] ;
    schema:CountryCode [
        schema:name "{countrycode}" ;
    ] ;
    schema:State [
        schema:name "{state}" ;
    ] ;
    schema:Language [
        schema:name "{language}" ;
    ] ;
    schema:PropertyValue [
        schema:name "votes" ;
        schema:value "{votes}"
    ] ;
    schema:PropertyValue [
        schema:name "negativevotes" ;
        schema:value "{negativevotes}"
    ] ;
    schema:PropertyValue [
        schema:name "lastchangetime" ;
        schema:value "{lastchangetime}"
    ] ;
    schema:PropertyValue [
        schema:name "ip" ;
        schema:value "{ip}"
    ] ;
    schema:PropertyValue [
        schema:name "codec" ;
        schema:value "{codec}"
    ] ;
    schema:PropertyValue [
        schema:name "bitrate" ;
        schema:value "{bitrate}"
    ] ;
    schema:PropertyValue [
        schema:name "hls" ;
        schema:value "{hls}"
    ] ;
    schema:PropertyValue [
        schema:name "lastcheckok" ;
        schema:value "{lastcheckok}"
    ] ;
    schema:PropertyValue [
        schema:name "lastchecktime" ;
        schema:value "{lastchecktime}"
    ] ;
    schema:PropertyValue [
        schema:name "lastcheckoktime" ;
        schema:value "{lastcheckoktime}"
    ] ;
    schema:PropertyValue [
        schema:name "clicktimestamp" ;
        schema:value "{clicktimestamp}"
    ] ;
    schema:PropertyValue [
        schema:name "clickcount" ;
        schema:value "{clickcount}"
    ] ;
    schema:PropertyValue [
        schema:name "clicktrend" ;
        schema:value "{clicktrend}"
    ] ;
    .{newline}"#,
            id = self.id,
            stationuuid = self.stationuuid,
            changeuuid = self.changeuuid,
            name = self.name,
            url = self.url,
            lastchangetime = self.lastchecktime,
            lastchecktime = self.lastchecktime,
            lastcheckoktime = self.lastcheckoktime,
            clicktimestamp = self.clicktimestamp,
            homepage = self.homepage,
            favicon = self.favicon,
            country = self.country,
            countrycode = self.countrycode,
            state = self.state,
            language = self.language,
            votes = self.votes,
            negativevotes = self.negativevotes,
            ip = self.ip,
            codec = self.codec,
            bitrate = self.bitrate,
            hls = self.hls,
            lastcheckok = self.lastcheckok,
            clickcount = self.clickcount,
            clicktrend = self.clicktrend,
            newline = "\r\n\r\n"
        )
    }

    pub fn serialize_to_ttl(list: Vec<Station>) -> String {
        let mut j = String::with_capacity(200 * list.len());

        j.push_str(
            r#"@prefix dcterms: <http://purl.org/dc/terms/> .
    @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    @prefix schema: <http://schema.org/> .
    @prefix wdrs: <https://www.w3.org/2007/05/powder-s#> .
    "#,
        );

        for entry in list {
            let x = entry.serialize_to_ttl_single();
            j.push_str(&x);
        }

        j
    }

    pub fn get_response(list : Vec<Station>, format : &str) -> rouille::Response {
        match format {
            "json" => {
                let j = serde_json::to_string(&list).unwrap();
                rouille::Response::text(j).with_no_cache().with_unique_header("Content-Type","application/json")
            },
            "xml" => {
                let j = Station::serialize_station_list(list).unwrap();
                rouille::Response::text(j).with_no_cache().with_unique_header("Content-Type","text/xml")
            },
            "m3u" => {
                let j = Station::serialize_to_m3u(list, false);
                rouille::Response::text(j).with_no_cache().with_unique_header("Content-Type","audio/mpegurl").with_unique_header("Content-Disposition", r#"inline; filename="playlist.m3u""#)
            },
            "pls" => {
                let j = Station::serialize_to_pls(list, false);
                rouille::Response::text(j).with_no_cache().with_unique_header("Content-Type","audio/x-scpls").with_unique_header("Content-Disposition", r#"inline; filename="playlist.pls""#)
            },
            "xspf" => {
                let j = Station::serialize_to_xspf(list).unwrap();
                rouille::Response::text(j).with_unique_header("Content-Type","application/xspf+xml").with_unique_header("Content-Disposition", r#"inline; filename="playlist.xspf""#)
            },
            "ttl" => {
                let j = Station::serialize_to_ttl(list);
                rouille::Response::text(j).with_no_cache().with_unique_header("Content-Type","text/turtle")
            },
            _ => rouille::Response::empty_406()
        }
    }
}

impl From<&StationHistoryCurrent> for Station {
    fn from(item: &StationHistoryCurrent) -> Self {
        Station {
            id: 0,
            changeuuid: item.changeuuid.clone(),
            stationuuid: item.stationuuid.clone(),
            name: item.name.clone(),
            url: item.url.clone(),
            homepage: item.homepage.clone(),
            favicon: item.favicon.clone(),
            tags: item.tags.clone(),
            country: item.country.clone(),
            countrycode: item.countrycode.clone(),
            state: item.state.clone(),
            language: item.language.clone(),
            votes: item.votes,
            negativevotes: item.negativevotes,
            lastchangetime: item.lastchangetime.clone(),
            ip: item.ip.clone(),
            bitrate: 0,
            clickcount: 0,
            clicktimestamp: String::from(""),
            clicktrend: 0,
            codec: String::from(""),
            hls: 0,
            lastcheckok: 0,
            lastcheckoktime: String::from(""),
            lastchecktime: String::from(""),
            urlcache: String::from(""),
        }
    }
}