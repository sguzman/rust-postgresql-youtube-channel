mod lib;


fn main() {
    let conn = lib::connect("root", "", "localhost", 5432, "youtube");

    for row in &conn.query("SELECT channel_id, id FROM youtube.channels.channels", &[]).unwrap() {
        let chan = lib::Channel {
            channel_serial: row.get(0),
            id: row.get(1)
        };
        println!("Found person {} {}", chan.id, chan.channel_serial);
    }

    conn.finish().unwrap()
}