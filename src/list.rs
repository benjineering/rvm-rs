use crate::github::get_available_versions;

pub async fn list_versions() {
	let ruby_versions = get_available_versions().await.unwrap();

	for ruby_version in &ruby_versions {
		println!("{}", ruby_version.version);
	}
}
