mod org_bootc;

use crate::org_bootc::{VarlinkInterface, Call_Status};
use varlink::Result;
use std::process::Command;

struct OrgBootc;

impl VarlinkInterface for OrgBootc {
    fn status(&self, call: &mut dyn Call_Status) -> Result<()> {
        println!("Running: 'bootc status'");
        let mut cmd = Command::new("bootc");
        cmd.arg("status");
        let Ok(output) = cmd.output() else {
            println!("failed to execute process");
            return call.reply("".to_string());
        };
        let s = match String::from_utf8(output.stdout) {
            Err(e) => {
                println!("non utf8 output: {e}");
                return call.reply("".to_string());
            },
            Ok(r) => r
        };
        return call.reply(s);
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let org_bootc = OrgBootc;
    let org_bootc_interface = org_bootc::new(Box::new(org_bootc));

    let service = varlink::VarlinkService::new(
        "org.varlink",
        "bootc",
        "0.1",
        "http://varlink.org",
        vec![
            Box::new(org_bootc_interface),
        ],
    );

    varlink::listen(service, &args[1],
        &varlink::ListenConfig {
            idle_timeout: 0,
            ..Default::default()
        },
    ).unwrap();
}
