#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use openapi_client::{Api, ApiNoContext, Client, ContextWrapperExt, models,
                      CreateAlienwareAuroraR15GamingDesktopResponse,
                      CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse,
                      DeleteAlienwareAuroraR15GamingDesktopResponse,
                      GetAlienwareAuroraR15GamingDesktopByNameResponse,
                      UpdateAlienwareAuroraR15GamingDesktopResponse,
                      CreateCatResponse,
                      CreateCatsWithListInputResponse,
                      DeleteCatResponse,
                      GetCatByNameResponse,
                      UpdateCatResponse,
                      CreateDell274kUhdMonitorS2721qsResponse,
                      CreateDell274kUhdMonitorS2721qssWithListInputResponse,
                      DeleteDell274kUhdMonitorS2721qsResponse,
                      GetDell274kUhdMonitorS2721qsByNameResponse,
                      UpdateDell274kUhdMonitorS2721qsResponse,
                      CreateDogResponse,
                      CreateDogsWithListInputResponse,
                      DeleteDogResponse,
                      GetDogByNameResponse,
                      UpdateDogResponse,
                      CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse,
                      CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse,
                      DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse,
                      GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse,
                      UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse,
                      CreateUserResponse,
                      CreateUsersWithListInputResponse,
                      DeleteUserResponse,
                      GetUserByNameResponse,
                      LoginUserResponse,
                      LogoutUserResponse,
                      UpdateUserResponse,
                     };
use clap::{App, Arg};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "CreateAlienwareAuroraR15GamingDesktop",
                "CreateAlienwareAuroraR15GamingDesktopsWithListInput",
                "DeleteAlienwareAuroraR15GamingDesktop",
                "GetAlienwareAuroraR15GamingDesktopByName",
                "UpdateAlienwareAuroraR15GamingDesktop",
                "CreateCat",
                "CreateCatsWithListInput",
                "DeleteCat",
                "GetCatByName",
                "UpdateCat",
                "CreateDell274kUhdMonitorS2721qs",
                "CreateDell274kUhdMonitorS2721qssWithListInput",
                "DeleteDell274kUhdMonitorS2721qs",
                "GetDell274kUhdMonitorS2721qsByName",
                "UpdateDell274kUhdMonitorS2721qs",
                "CreateDog",
                "CreateDogsWithListInput",
                "DeleteDog",
                "GetDogByName",
                "UpdateDog",
                "CreateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor",
                "CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInput",
                "DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9Processor",
                "GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByName",
                "UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor",
                "CreateUser",
                "CreateUsersWithListInput",
                "DeleteUser",
                "GetUserByName",
                "LoginUser",
                "LogoutUser",
                "UpdateUser",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("CreateAlienwareAuroraR15GamingDesktop") => {
            let result = rt.block_on(client.create_alienware_aurora_r15_gaming_desktop(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateAlienwareAuroraR15GamingDesktopsWithListInput") => {
            let result = rt.block_on(client.create_alienware_aurora_r15_gaming_desktops_with_list_input(
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteAlienwareAuroraR15GamingDesktop") => {
            let result = rt.block_on(client.delete_alienware_aurora_r15_gaming_desktop(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAlienwareAuroraR15GamingDesktopByName") => {
            let result = rt.block_on(client.get_alienware_aurora_r15_gaming_desktop_by_name(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("UpdateAlienwareAuroraR15GamingDesktop") => {
            let result = rt.block_on(client.update_alienware_aurora_r15_gaming_desktop(
                  "name_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateCat") => {
            let result = rt.block_on(client.create_cat(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateCatsWithListInput") => {
            let result = rt.block_on(client.create_cats_with_list_input(
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteCat") => {
            let result = rt.block_on(client.delete_cat(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetCatByName") => {
            let result = rt.block_on(client.get_cat_by_name(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("UpdateCat") => {
            let result = rt.block_on(client.update_cat(
                  "name_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateDell274kUhdMonitorS2721qs") => {
            let result = rt.block_on(client.create_dell274k_uhd_monitor_s2721qs(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateDell274kUhdMonitorS2721qssWithListInput") => {
            let result = rt.block_on(client.create_dell274k_uhd_monitor_s2721qss_with_list_input(
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteDell274kUhdMonitorS2721qs") => {
            let result = rt.block_on(client.delete_dell274k_uhd_monitor_s2721qs(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetDell274kUhdMonitorS2721qsByName") => {
            let result = rt.block_on(client.get_dell274k_uhd_monitor_s2721qs_by_name(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("UpdateDell274kUhdMonitorS2721qs") => {
            let result = rt.block_on(client.update_dell274k_uhd_monitor_s2721qs(
                  "name_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateDog") => {
            let result = rt.block_on(client.create_dog(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateDogsWithListInput") => {
            let result = rt.block_on(client.create_dogs_with_list_input(
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteDog") => {
            let result = rt.block_on(client.delete_dog(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetDogByName") => {
            let result = rt.block_on(client.get_dog_by_name(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("UpdateDog") => {
            let result = rt.block_on(client.update_dog(
                  "name_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor") => {
            let result = rt.block_on(client.create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInput") => {
            let result = rt.block_on(client.create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9Processor") => {
            let result = rt.block_on(client.delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByName") => {
            let result = rt.block_on(client.get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(
                  "name_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor") => {
            let result = rt.block_on(client.update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
                  "name_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateUser") => {
            let result = rt.block_on(client.create_user(
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateUsersWithListInput") => {
            let result = rt.block_on(client.create_users_with_list_input(
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteUser") => {
            let result = rt.block_on(client.delete_user(
                  "username_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetUserByName") => {
            let result = rt.block_on(client.get_user_by_name(
                  "username_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("LoginUser") => {
            let result = rt.block_on(client.login_user(
                  Some("username_example".to_string()),
                  Some("password_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("LogoutUser") => {
            let result = rt.block_on(client.logout_user(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("UpdateUser") => {
            let result = rt.block_on(client.update_user(
                  "username_example".to_string(),
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
