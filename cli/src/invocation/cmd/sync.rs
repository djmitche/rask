use failure::Fallible;
use taskchampion::{server::Server, Replica};
use termcolor::WriteColor;

pub(crate) fn execute<W: WriteColor>(
    w: &mut W,
    replica: &mut Replica,
    server: &mut Box<dyn Server>,
) -> Fallible<()> {
    replica.sync(server)?;
    write!(w, "sync complete.\n")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::invocation::cmd::test::*;
    use tempdir::TempDir;

    #[test]
    fn test_add() {
        let mut w = test_writer();
        let mut replica = test_replica();
        let server_dir = TempDir::new("test").unwrap();
        let mut server = test_server(&server_dir);

        // Note that the details of the actual sync are tested thoroughly in the taskchampion crate
        execute(&mut w, &mut replica, &mut server).unwrap();
        assert_eq!(&w.into_string(), "sync complete.\n")
    }
}