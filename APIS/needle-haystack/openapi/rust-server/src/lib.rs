#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::blacklisted_name)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/openapi-jaxrs-server-1.0.0/api/v3";
pub const API_VERSION: &str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateAlienwareAuroraR15GamingDesktopResponse {
    /// successful operation
    SuccessfulOperation
    (models::AlienwareAuroraR15GamingDesktop)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse {
    /// Successful operation
    SuccessfulOperation
    (models::AlienwareAuroraR15GamingDesktop)
    ,
    /// successful operation
    SuccessfulOperation_2
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteAlienwareAuroraR15GamingDesktopResponse {
    /// Invalid AlienwareAuroraR15GamingDesktop variable supplied
    InvalidAlienwareAuroraR
    ,
    /// AlienwareAuroraR15GamingDesktop not found
    AlienwareAuroraR
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAlienwareAuroraR15GamingDesktopByNameResponse {
    /// successful operation
    SuccessfulOperation
    (models::AlienwareAuroraR15GamingDesktop)
    ,
    /// Invalid AlienwareAuroraR15GamingDesktop variable supplied
    InvalidAlienwareAuroraR
    ,
    /// AlienwareAuroraR15GamingDesktop not found
    AlienwareAuroraR
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateAlienwareAuroraR15GamingDesktopResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateCatResponse {
    /// successful operation
    SuccessfulOperation
    (models::Cat)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateCatsWithListInputResponse {
    /// Successful operation
    SuccessfulOperation
    (models::Cat)
    ,
    /// successful operation
    SuccessfulOperation_2
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteCatResponse {
    /// Invalid Cat variable supplied
    InvalidCatVariableSupplied
    ,
    /// Cat not found
    CatNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCatByNameResponse {
    /// successful operation
    SuccessfulOperation
    (models::Cat)
    ,
    /// Invalid Cat variable supplied
    InvalidCatVariableSupplied
    ,
    /// Cat not found
    CatNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateCatResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateDell274kUhdMonitorS2721qsResponse {
    /// successful operation
    SuccessfulOperation
    (models::Dell274kUhdMonitorS2721qs)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateDell274kUhdMonitorS2721qssWithListInputResponse {
    /// Successful operation
    SuccessfulOperation
    (models::Dell274kUhdMonitorS2721qs)
    ,
    /// successful operation
    SuccessfulOperation_2
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteDell274kUhdMonitorS2721qsResponse {
    /// Invalid Dell274kUhdMonitor-S2721qs variable supplied
    InvalidDell
    ,
    /// Dell274kUhdMonitor-S2721qs not found
    Dell
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetDell274kUhdMonitorS2721qsByNameResponse {
    /// successful operation
    SuccessfulOperation
    (models::Dell274kUhdMonitorS2721qs)
    ,
    /// Invalid Dell274kUhdMonitor-S2721qs variable supplied
    InvalidDell
    ,
    /// Dell274kUhdMonitor-S2721qs not found
    Dell
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateDell274kUhdMonitorS2721qsResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateDogResponse {
    /// successful operation
    SuccessfulOperation
    (models::Dog)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateDogsWithListInputResponse {
    /// Successful operation
    SuccessfulOperation
    (models::Dog)
    ,
    /// successful operation
    SuccessfulOperation_2
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteDogResponse {
    /// Invalid Dog variable supplied
    InvalidDogVariableSupplied
    ,
    /// Dog not found
    DogNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetDogByNameResponse {
    /// successful operation
    SuccessfulOperation
    (models::Dog)
    ,
    /// Invalid Dog variable supplied
    InvalidDogVariableSupplied
    ,
    /// Dog not found
    DogNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateDogResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse {
    /// successful operation
    SuccessfulOperation
    (models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse {
    /// Successful operation
    SuccessfulOperation
    (models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor)
    ,
    /// successful operation
    SuccessfulOperation_2
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse {
    /// Invalid LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor variable supplied
    InvalidLenovoLegionTower
    ,
    /// LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor not found
    LenovoLegionTower
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse {
    /// successful operation
    SuccessfulOperation
    (models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor)
    ,
    /// Invalid LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor variable supplied
    InvalidLenovoLegionTower
    ,
    /// LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor not found
    LenovoLegionTower
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateUserResponse {
    /// successful operation
    SuccessfulOperation
    (models::User)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateUsersWithListInputResponse {
    /// Successful operation
    SuccessfulOperation
    (models::User)
    ,
    /// successful operation
    SuccessfulOperation_2
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteUserResponse {
    /// Invalid username supplied
    InvalidUsernameSupplied
    ,
    /// User not found
    UserNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetUserByNameResponse {
    /// successful operation
    SuccessfulOperation
    (models::User)
    ,
    /// Invalid username supplied
    InvalidUsernameSupplied
    ,
    /// User not found
    UserNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum LoginUserResponse {
    /// successful operation
    SuccessfulOperation
    {
        body: String,
        x_rate_limit:
        Option<
        i32
        >
        ,
        x_expires_after:
        Option<
        chrono::DateTime::<chrono::Utc>
        >
    }
    ,
    /// Invalid username/password supplied
    InvalidUsername
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LogoutUserResponse {
    /// successful operation
    SuccessfulOperation
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateUserResponse {
    /// successful operation
    SuccessfulOperation
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Create AlienwareAuroraR15GamingDesktop
    async fn create_alienware_aurora_r15_gaming_desktop(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        context: &C) -> Result<CreateAlienwareAuroraR15GamingDesktopResponse, ApiError>;

    /// Creates list of AlienwareAuroraR15GamingDesktop with given input array
    async fn create_alienware_aurora_r15_gaming_desktops_with_list_input(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<&Vec<models::AlienwareAuroraR15GamingDesktop>>,
        context: &C) -> Result<CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse, ApiError>;

    /// Delete alienwareAuroraR15GamingDesktop
    async fn delete_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        context: &C) -> Result<DeleteAlienwareAuroraR15GamingDesktopResponse, ApiError>;

    /// Get alienwareAuroraR15GamingDesktop by name
    async fn get_alienware_aurora_r15_gaming_desktop_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetAlienwareAuroraR15GamingDesktopByNameResponse, ApiError>;

    /// Update alienwareAuroraR15GamingDesktop
    async fn update_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        context: &C) -> Result<UpdateAlienwareAuroraR15GamingDesktopResponse, ApiError>;

    /// Create Cat
    async fn create_cat(
        &self,
        cat: Option<models::Cat>,
        context: &C) -> Result<CreateCatResponse, ApiError>;

    /// Creates list of Cat with given input array
    async fn create_cats_with_list_input(
        &self,
        cat: Option<&Vec<models::Cat>>,
        context: &C) -> Result<CreateCatsWithListInputResponse, ApiError>;

    /// Delete cat
    async fn delete_cat(
        &self,
        name: String,
        context: &C) -> Result<DeleteCatResponse, ApiError>;

    /// Get cat by name
    async fn get_cat_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetCatByNameResponse, ApiError>;

    /// Update cat
    async fn update_cat(
        &self,
        name: String,
        cat: Option<models::Cat>,
        context: &C) -> Result<UpdateCatResponse, ApiError>;

    /// Create Dell274kUhdMonitor-S2721qs
    async fn create_dell274k_uhd_monitor_s2721qs(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        context: &C) -> Result<CreateDell274kUhdMonitorS2721qsResponse, ApiError>;

    /// Creates list of Dell274kUhdMonitor-S2721qs with given input array
    async fn create_dell274k_uhd_monitor_s2721qss_with_list_input(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<&Vec<models::Dell274kUhdMonitorS2721qs>>,
        context: &C) -> Result<CreateDell274kUhdMonitorS2721qssWithListInputResponse, ApiError>;

    /// Delete dell274kUhDMonitor-S2721QS
    async fn delete_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        context: &C) -> Result<DeleteDell274kUhdMonitorS2721qsResponse, ApiError>;

    /// Get dell274kUhDMonitor-S2721QS by name
    async fn get_dell274k_uhd_monitor_s2721qs_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetDell274kUhdMonitorS2721qsByNameResponse, ApiError>;

    /// Update dell274kUhDMonitor-S2721QS
    async fn update_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        context: &C) -> Result<UpdateDell274kUhdMonitorS2721qsResponse, ApiError>;

    /// Create Dog
    async fn create_dog(
        &self,
        dog: Option<models::Dog>,
        context: &C) -> Result<CreateDogResponse, ApiError>;

    /// Creates list of Dog with given input array
    async fn create_dogs_with_list_input(
        &self,
        dog: Option<&Vec<models::Dog>>,
        context: &C) -> Result<CreateDogsWithListInputResponse, ApiError>;

    /// Delete dog
    async fn delete_dog(
        &self,
        name: String,
        context: &C) -> Result<DeleteDogResponse, ApiError>;

    /// Get dog by name
    async fn get_dog_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetDogByNameResponse, ApiError>;

    /// Update dog
    async fn update_dog(
        &self,
        name: String,
        dog: Option<models::Dog>,
        context: &C) -> Result<UpdateDogResponse, ApiError>;

    /// Create LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        context: &C) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>;

    /// Creates list of LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor with given input array
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<&Vec<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>>,
        context: &C) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse, ApiError>;

    /// Delete lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        context: &C) -> Result<DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>;

    /// Get lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor by name
    async fn get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse, ApiError>;

    /// Update lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        context: &C) -> Result<UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>;

    /// Create user
    async fn create_user(
        &self,
        user: Option<models::User>,
        context: &C) -> Result<CreateUserResponse, ApiError>;

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: Option<&Vec<models::User>>,
        context: &C) -> Result<CreateUsersWithListInputResponse, ApiError>;

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        context: &C) -> Result<DeleteUserResponse, ApiError>;

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        context: &C) -> Result<GetUserByNameResponse, ApiError>;

    /// Logs user into the system
    async fn login_user(
        &self,
        username: Option<String>,
        password: Option<String>,
        context: &C) -> Result<LoginUserResponse, ApiError>;

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        context: &C) -> Result<LogoutUserResponse, ApiError>;

    /// Update user
    async fn update_user(
        &self,
        username: String,
        user: Option<models::User>,
        context: &C) -> Result<UpdateUserResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Create AlienwareAuroraR15GamingDesktop
    async fn create_alienware_aurora_r15_gaming_desktop(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        ) -> Result<CreateAlienwareAuroraR15GamingDesktopResponse, ApiError>;

    /// Creates list of AlienwareAuroraR15GamingDesktop with given input array
    async fn create_alienware_aurora_r15_gaming_desktops_with_list_input(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<&Vec<models::AlienwareAuroraR15GamingDesktop>>,
        ) -> Result<CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse, ApiError>;

    /// Delete alienwareAuroraR15GamingDesktop
    async fn delete_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        ) -> Result<DeleteAlienwareAuroraR15GamingDesktopResponse, ApiError>;

    /// Get alienwareAuroraR15GamingDesktop by name
    async fn get_alienware_aurora_r15_gaming_desktop_by_name(
        &self,
        name: String,
        ) -> Result<GetAlienwareAuroraR15GamingDesktopByNameResponse, ApiError>;

    /// Update alienwareAuroraR15GamingDesktop
    async fn update_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        ) -> Result<UpdateAlienwareAuroraR15GamingDesktopResponse, ApiError>;

    /// Create Cat
    async fn create_cat(
        &self,
        cat: Option<models::Cat>,
        ) -> Result<CreateCatResponse, ApiError>;

    /// Creates list of Cat with given input array
    async fn create_cats_with_list_input(
        &self,
        cat: Option<&Vec<models::Cat>>,
        ) -> Result<CreateCatsWithListInputResponse, ApiError>;

    /// Delete cat
    async fn delete_cat(
        &self,
        name: String,
        ) -> Result<DeleteCatResponse, ApiError>;

    /// Get cat by name
    async fn get_cat_by_name(
        &self,
        name: String,
        ) -> Result<GetCatByNameResponse, ApiError>;

    /// Update cat
    async fn update_cat(
        &self,
        name: String,
        cat: Option<models::Cat>,
        ) -> Result<UpdateCatResponse, ApiError>;

    /// Create Dell274kUhdMonitor-S2721qs
    async fn create_dell274k_uhd_monitor_s2721qs(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        ) -> Result<CreateDell274kUhdMonitorS2721qsResponse, ApiError>;

    /// Creates list of Dell274kUhdMonitor-S2721qs with given input array
    async fn create_dell274k_uhd_monitor_s2721qss_with_list_input(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<&Vec<models::Dell274kUhdMonitorS2721qs>>,
        ) -> Result<CreateDell274kUhdMonitorS2721qssWithListInputResponse, ApiError>;

    /// Delete dell274kUhDMonitor-S2721QS
    async fn delete_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        ) -> Result<DeleteDell274kUhdMonitorS2721qsResponse, ApiError>;

    /// Get dell274kUhDMonitor-S2721QS by name
    async fn get_dell274k_uhd_monitor_s2721qs_by_name(
        &self,
        name: String,
        ) -> Result<GetDell274kUhdMonitorS2721qsByNameResponse, ApiError>;

    /// Update dell274kUhDMonitor-S2721QS
    async fn update_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        ) -> Result<UpdateDell274kUhdMonitorS2721qsResponse, ApiError>;

    /// Create Dog
    async fn create_dog(
        &self,
        dog: Option<models::Dog>,
        ) -> Result<CreateDogResponse, ApiError>;

    /// Creates list of Dog with given input array
    async fn create_dogs_with_list_input(
        &self,
        dog: Option<&Vec<models::Dog>>,
        ) -> Result<CreateDogsWithListInputResponse, ApiError>;

    /// Delete dog
    async fn delete_dog(
        &self,
        name: String,
        ) -> Result<DeleteDogResponse, ApiError>;

    /// Get dog by name
    async fn get_dog_by_name(
        &self,
        name: String,
        ) -> Result<GetDogByNameResponse, ApiError>;

    /// Update dog
    async fn update_dog(
        &self,
        name: String,
        dog: Option<models::Dog>,
        ) -> Result<UpdateDogResponse, ApiError>;

    /// Create LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        ) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>;

    /// Creates list of LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor with given input array
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<&Vec<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>>,
        ) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse, ApiError>;

    /// Delete lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        ) -> Result<DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>;

    /// Get lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor by name
    async fn get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(
        &self,
        name: String,
        ) -> Result<GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse, ApiError>;

    /// Update lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        ) -> Result<UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>;

    /// Create user
    async fn create_user(
        &self,
        user: Option<models::User>,
        ) -> Result<CreateUserResponse, ApiError>;

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: Option<&Vec<models::User>>,
        ) -> Result<CreateUsersWithListInputResponse, ApiError>;

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        ) -> Result<DeleteUserResponse, ApiError>;

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        ) -> Result<GetUserByNameResponse, ApiError>;

    /// Logs user into the system
    async fn login_user(
        &self,
        username: Option<String>,
        password: Option<String>,
        ) -> Result<LoginUserResponse, ApiError>;

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        ) -> Result<LogoutUserResponse, ApiError>;

    /// Update user
    async fn update_user(
        &self,
        username: String,
        user: Option<models::User>,
        ) -> Result<UpdateUserResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Create AlienwareAuroraR15GamingDesktop
    async fn create_alienware_aurora_r15_gaming_desktop(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        ) -> Result<CreateAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_alienware_aurora_r15_gaming_desktop(alienware_aurora_r15_gaming_desktop, &context).await
    }

    /// Creates list of AlienwareAuroraR15GamingDesktop with given input array
    async fn create_alienware_aurora_r15_gaming_desktops_with_list_input(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<&Vec<models::AlienwareAuroraR15GamingDesktop>>,
        ) -> Result<CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_alienware_aurora_r15_gaming_desktops_with_list_input(alienware_aurora_r15_gaming_desktop, &context).await
    }

    /// Delete alienwareAuroraR15GamingDesktop
    async fn delete_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        ) -> Result<DeleteAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_alienware_aurora_r15_gaming_desktop(name, &context).await
    }

    /// Get alienwareAuroraR15GamingDesktop by name
    async fn get_alienware_aurora_r15_gaming_desktop_by_name(
        &self,
        name: String,
        ) -> Result<GetAlienwareAuroraR15GamingDesktopByNameResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_alienware_aurora_r15_gaming_desktop_by_name(name, &context).await
    }

    /// Update alienwareAuroraR15GamingDesktop
    async fn update_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        ) -> Result<UpdateAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_alienware_aurora_r15_gaming_desktop(name, alienware_aurora_r15_gaming_desktop, &context).await
    }

    /// Create Cat
    async fn create_cat(
        &self,
        cat: Option<models::Cat>,
        ) -> Result<CreateCatResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_cat(cat, &context).await
    }

    /// Creates list of Cat with given input array
    async fn create_cats_with_list_input(
        &self,
        cat: Option<&Vec<models::Cat>>,
        ) -> Result<CreateCatsWithListInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_cats_with_list_input(cat, &context).await
    }

    /// Delete cat
    async fn delete_cat(
        &self,
        name: String,
        ) -> Result<DeleteCatResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_cat(name, &context).await
    }

    /// Get cat by name
    async fn get_cat_by_name(
        &self,
        name: String,
        ) -> Result<GetCatByNameResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cat_by_name(name, &context).await
    }

    /// Update cat
    async fn update_cat(
        &self,
        name: String,
        cat: Option<models::Cat>,
        ) -> Result<UpdateCatResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_cat(name, cat, &context).await
    }

    /// Create Dell274kUhdMonitor-S2721qs
    async fn create_dell274k_uhd_monitor_s2721qs(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        ) -> Result<CreateDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_dell274k_uhd_monitor_s2721qs(dell274k_uhd_monitor_s2721qs, &context).await
    }

    /// Creates list of Dell274kUhdMonitor-S2721qs with given input array
    async fn create_dell274k_uhd_monitor_s2721qss_with_list_input(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<&Vec<models::Dell274kUhdMonitorS2721qs>>,
        ) -> Result<CreateDell274kUhdMonitorS2721qssWithListInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_dell274k_uhd_monitor_s2721qss_with_list_input(dell274k_uhd_monitor_s2721qs, &context).await
    }

    /// Delete dell274kUhDMonitor-S2721QS
    async fn delete_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        ) -> Result<DeleteDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_dell274k_uhd_monitor_s2721qs(name, &context).await
    }

    /// Get dell274kUhDMonitor-S2721QS by name
    async fn get_dell274k_uhd_monitor_s2721qs_by_name(
        &self,
        name: String,
        ) -> Result<GetDell274kUhdMonitorS2721qsByNameResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_dell274k_uhd_monitor_s2721qs_by_name(name, &context).await
    }

    /// Update dell274kUhDMonitor-S2721QS
    async fn update_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        ) -> Result<UpdateDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_dell274k_uhd_monitor_s2721qs(name, dell274k_uhd_monitor_s2721qs, &context).await
    }

    /// Create Dog
    async fn create_dog(
        &self,
        dog: Option<models::Dog>,
        ) -> Result<CreateDogResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_dog(dog, &context).await
    }

    /// Creates list of Dog with given input array
    async fn create_dogs_with_list_input(
        &self,
        dog: Option<&Vec<models::Dog>>,
        ) -> Result<CreateDogsWithListInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_dogs_with_list_input(dog, &context).await
    }

    /// Delete dog
    async fn delete_dog(
        &self,
        name: String,
        ) -> Result<DeleteDogResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_dog(name, &context).await
    }

    /// Get dog by name
    async fn get_dog_by_name(
        &self,
        name: String,
        ) -> Result<GetDogByNameResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_dog_by_name(name, &context).await
    }

    /// Update dog
    async fn update_dog(
        &self,
        name: String,
        dog: Option<models::Dog>,
        ) -> Result<UpdateDogResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_dog(name, dog, &context).await
    }

    /// Create LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        ) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor, &context).await
    }

    /// Creates list of LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor with given input array
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<&Vec<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>>,
        ) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor, &context).await
    }

    /// Delete lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        ) -> Result<DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(name, &context).await
    }

    /// Get lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor by name
    async fn get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(
        &self,
        name: String,
        ) -> Result<GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(name, &context).await
    }

    /// Update lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        ) -> Result<UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(name, lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor, &context).await
    }

    /// Create user
    async fn create_user(
        &self,
        user: Option<models::User>,
        ) -> Result<CreateUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_user(user, &context).await
    }

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: Option<&Vec<models::User>>,
        ) -> Result<CreateUsersWithListInputResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_users_with_list_input(user, &context).await
    }

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        ) -> Result<DeleteUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_user(username, &context).await
    }

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        ) -> Result<GetUserByNameResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_user_by_name(username, &context).await
    }

    /// Logs user into the system
    async fn login_user(
        &self,
        username: Option<String>,
        password: Option<String>,
        ) -> Result<LoginUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().login_user(username, password, &context).await
    }

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        ) -> Result<LogoutUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().logout_user(&context).await
    }

    /// Update user
    async fn update_user(
        &self,
        username: String,
        user: Option<models::User>,
        ) -> Result<UpdateUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_user(username, user, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
