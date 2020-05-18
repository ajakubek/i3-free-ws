extern crate i3ipc;

use i3ipc::I3Connection;

fn main() {
    let mut connection = match I3Connection::connect() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("i3 connection failed: {}", err);
            return;
        }
    };

    let mut workspaces = match connection.get_workspaces() {
        Ok(ws) => ws.workspaces,
        Err(err) => {
            eprintln!("failed to obtain i3 workspaces: {}", err);
            return;
        }
    };

    workspaces.sort_unstable_by_key(|ws| ws.num);

    for num in 1..11 {
        if workspaces.binary_search_by(|ws| ws.num.cmp(&num)).is_err() {
            if let Err(err) = connection.run_command(&format!("workspace {}", num)) {
                eprintln!("failed to switch i3 workspace: {}", err);
            }
            break;
        }
    }
}
