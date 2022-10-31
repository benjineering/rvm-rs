use crate::version::VersionId;

pub struct Installer {
	version_string: VersionId,
}

impl Installer {
	pub fn new(version_string: VersionId) -> Installer {
		Installer { version_string }
	}

	pub fn run(&self) {
		println!("{:?}", self.version_string);
	}
}
