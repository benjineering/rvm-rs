use crate::github::get_available_versions;

pub async fn list_versions() {
	let versions = get_available_versions().await.unwrap();

	for version in &versions {
		println!("{}", version.id);
	}
}
