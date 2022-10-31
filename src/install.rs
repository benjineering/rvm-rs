use crate::version::VersionId;

pub async fn install(version_id: VersionId) {
	println!("{:?}", version_id);
}
