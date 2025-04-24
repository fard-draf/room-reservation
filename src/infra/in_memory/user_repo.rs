// #[cfg(test)]
// mod test {

//     use crate::{
//         domain::{User, UserName},
//         error::{ErrRepo, ErrService, ErrUser},
//         features::user::{repo::UserRepo, service::UserService},
//         infra::in_memory::in_memo_repo::InMemoryRepo,
//     };
//     use async_trait::async_trait;
//     use uuid::Uuid;

//     #[async_trait]
//     impl UserRepo for InMemoryRepo<User> {
//         async fn insert_user(&self, user: &User) -> Result<User, ErrService> {
//             let vec = self.get_all_users().await?;
//             if vec.iter().any(|x| x.user_name == user.user_name) {
//                 return Err(ErrService::User(ErrUser::AlreadyExist));
//             }
//             self.repo
//                 .lock()
//                 .unwrap()
//                 .insert(user.user_id.id, user.clone());
//             Ok(user.clone())
//         }
//         async fn delete_user_by_name(&self, user_name: UserName) -> Result<bool, ErrService> {
//             let mut repo = self.repo.lock().unwrap();
//             if let Some(key) = repo.iter().find_map(|(k, v)| {
//                 if v.user_name == user_name {
//                     Some(*k)
//                 } else {
//                     None
//                 }
//             }) {
//                 repo.remove(&key)
//                     .ok_or(ErrService::Repo(ErrRepo::UnableToDelete))?;
//             }
//             Ok(true)
//         }
//         async fn update_user(&self, id: Uuid, new_name: UserName) -> Result<User, ErrService> {
//             let mut repo = self.repo.lock().unwrap();
//             let user: &mut User = repo
//                 .get_mut(&id)
//                 .ok_or(ErrService::User(ErrUser::UserNotFound))?;
//             user.user_name = new_name;

//             Ok(user.clone())
//         }
//         async fn get_all_users(&self) -> Result<Vec<User>, ErrService> {
//             let users = self.repo.lock().unwrap().values().cloned().collect();
//             Ok(users)
//         }

//         async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ErrService> {
//             let repo = self.repo.lock().unwrap();
//             Ok(repo.get(&id).cloned())
//         }
//     }

//     #[tokio::test]
//     async fn print_all_users() {
//         let repo = InMemoryRepo::new().await;
//         let service = UserService::new(repo);

//         assert!(service.add_user("Sophie").await.is_ok());
//         assert!(service.add_user("Jordan").await.is_ok());
//     }

//     #[tokio::test]
//     async fn add_and_list_user() {
//         let repo = InMemoryRepo::new().await;
//         let service = UserService::new(repo);

//         let user_ok1 = service.add_user("Sophie").await;
//         let user_ok2 = service.add_user("Jordan").await;

//         let user_err1 = service.add_user(" SOPHIE ").await; //already exist
//         let user_err2 = service.add_user("SoPhiE").await; //already exist
//         let user_err3 = service.add_user("A").await; //too short
//         let user_err4 = service //too long
//             .add_user("ABCDEFG       HIJKLMNO    PQRSTUVWXYZ")
//             .await;

//         assert!(user_ok1.is_ok());
//         assert!(user_ok2.is_ok());

//         assert!(user_err1.is_err());
//         assert!(user_err2.is_err());
//         assert!(user_err3.is_err());
//         assert!(user_err4.is_err());

//         assert!(service.is_exist_user("Jordan").await.is_ok());
//         assert!(service.is_exist_user("     JORDAN   ").await.is_ok());
//         assert!(!service.is_exist_user("Daniel").await.unwrap());
//     }

//     #[tokio::test]
//     async fn delete_user_by_name() {
//         let repo = InMemoryRepo::new().await;
//         let mut service = UserService::new(repo);

//         assert!(service.add_user("Sophie").await.is_ok());
//         assert!(service.add_user("Jordan").await.is_ok());

//         assert!(service.delete_user("Sophie").await.is_ok());
//     }

//     #[tokio::test]
//     async fn update_user_name() {
//         let repo = InMemoryRepo::new().await;
//         let service = UserService::new(repo);

//         assert!(service.add_user("Sophie").await.is_ok());
//         assert!(service.is_exist_user("Sophie").await.is_ok());
//         assert!(service.update_user("Sophie", "Alice").await.is_ok());
//         assert!(service.is_exist_user("Alice").await.is_ok());
//         assert!(service.update_user("ALICE", "CALISSE").await.is_ok());
//         assert!(service.is_exist_user("Calisse").await.is_ok());

//         assert!(service.update_user("Unexisting", "Bob").await.is_err());
//     }
// }
