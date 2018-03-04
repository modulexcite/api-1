/* Pi-hole: A black hole for Internet advertisements
*  (c) 2018 Pi-hole, LLC (https://pi-hole.net)
*  Network-wide ad blocking via your own hardware.
*
*  API
*  Blocking Status Endpoint
*
*  This file is copyright under the latest version of the EUPL.
*  Please see LICENSE file for your rights under this license. */

use config::{Config, PiholeFile};
use rocket::State;
use util;

/// Get the DNS blocking status
#[get("/dns/status")]
pub fn status(config: State<Config>) -> util::Reply {
    let file = config.read_file(PiholeFile::DnsmasqMainConfig);

    let status = if file.is_err() {
        // If we failed to open the file, then the status is unknown
        "unknown"
    } else {
        // Read the file to a buffer
        let mut buffer = String::new();
        file?.read_to_string(&mut buffer)?;

        // Check if the gravity.list line is disabled
        let disabled = buffer.lines()
            .filter(|line| *line == "#addn-hosts=/etc/pihole/gravity.list")
            .count();

        // Get the status string
        match disabled {
            0 => "enabled",
            1 => "disabled",
            _ => "unknown"
        }
    };

    util::reply_data(json!({
        "status": status
    }))
}

#[cfg(test)]
mod test {
    use config::PiholeFile;
    use testing::test_endpoint_config;

    #[test]
    fn test_status_enabled() {
        test_endpoint_config(
            "/admin/api/dns/status",
            PiholeFile::DnsmasqMainConfig,
            "addn-hosts=/etc/pihole/gravity.list".into(),
            json!({
                "data": {
                    "status": "enabled"
                },
                "errors": []
            })
        );
    }
}
