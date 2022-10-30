use crate::versions::VersionString;

pub struct Installer {
	version_string: VersionString,
}

impl Installer {
	pub fn new(version_string: VersionString) -> Installer {
		Installer { version_string }
	}

	pub fn run(&self) {
		println!("{:?}", self.version_string);
	}
}
