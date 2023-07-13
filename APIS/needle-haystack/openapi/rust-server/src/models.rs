#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "alienwareAuroraR15GamingDesktop")]
pub struct AlienwareAuroraR15GamingDesktop {
    /// This attribute is a variable named:NVidia Graphics
    #[serde(rename = "nVidiaGraphics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub n_vidia_graphics: Option<String>,

    /// This attribute is a variable named:HDMI Outputs
    #[serde(rename = "hDMIOutputs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub h_dmi_outputs: Option<i64>,

    /// This attribute is a variable named:Windows 10 Compatible
    #[serde(rename = "windows10Compatible")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub windows10_compatible: Option<bool>,

    /// This attribute is a variable named:AWCC Interface
    #[serde(rename = "aWCCInterface")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub a_wcc_interface: Option<String>,

    /// This attribute is a variable named:Multi Monitor Support
    #[serde(rename = "multiMonitorSupport")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub multi_monitor_support: Option<bool>,

    /// This attribute is a variable named:Heat Management System
    #[serde(rename = "heatManagementSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub heat_management_system: Option<bool>,

    /// This attribute is a variable named:1TB SSD Storage
    #[serde(rename = "1TBSsDStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub param_1_tbss_d_storage: Option<f64>,

    /// This attribute is a variable named:8th Gen Processor
    #[serde(rename = "8thGenProcessor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub param_8th_gen_processor: Option<String>,

    /// This attribute is a variable named:High Definition Audio Out
    #[serde(rename = "highDefinitionAudioOut")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub high_definition_audio_out: Option<bool>,

    #[serde(rename = "gPO-fb8")]
    pub g_po_fb8: i64,

    #[serde(rename = "gPO-e23")]
    pub g_po_e23: String,

    /// This attribute is a variable named:Liquid Cooling Option
    #[serde(rename = "liquidCoolingOption")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub liquid_cooling_option: Option<bool>,

    /// This attribute is a variable named:Built-in Wi-Fi
    #[serde(rename = "built-inWi-Fi")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub built_in_wi_fi: Option<bool>,

    /// This attribute is a variable named:Customizable Lighting
    #[serde(rename = "customizableLighting")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customizable_lighting: Option<bool>,

    /// This attribute is a variable named:Max RAM Capacity
    #[serde(rename = "maxRaMCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_ra_m_capacity: Option<i64>,

    /// This attribute is a variable named:7 USB Ports
    #[serde(rename = "7UsBPorts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub param_7_us_b_ports: Option<i64>,

    /// This attribute is a variable named:Smart Airflow Technology
    #[serde(rename = "smartAirflowTechnology")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub smart_airflow_technology: Option<bool>,

}

impl AlienwareAuroraR15GamingDesktop {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AlienwareAuroraR15GamingDesktop {
        AlienwareAuroraR15GamingDesktop {
            n_vidia_graphics: None,
            h_dmi_outputs: Some(0),
            windows10_compatible: Some(false),
            a_wcc_interface: None,
            multi_monitor_support: Some(false),
            heat_management_system: Some(false),
            param_1_tbss_d_storage: Some(0.0),
            param_8th_gen_processor: None,
            high_definition_audio_out: Some(false),
            g_po_fb8: 2244,
            g_po_e23: "2245".to_string(),
            liquid_cooling_option: Some(false),
            built_in_wi_fi: Some(false),
            customizable_lighting: Some(false),
            max_ra_m_capacity: Some(0),
            param_7_us_b_ports: Some(0),
            smart_airflow_technology: Some(false),
        }
    }
}

/// Converts the AlienwareAuroraR15GamingDesktop value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AlienwareAuroraR15GamingDesktop {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.n_vidia_graphics.as_ref().map(|n_vidia_graphics| {
                vec![
                    "nVidiaGraphics".to_string(),
                    n_vidia_graphics.to_string(),
                ].join(",")
            }),


            self.h_dmi_outputs.as_ref().map(|h_dmi_outputs| {
                vec![
                    "hDMIOutputs".to_string(),
                    h_dmi_outputs.to_string(),
                ].join(",")
            }),


            self.windows10_compatible.as_ref().map(|windows10_compatible| {
                vec![
                    "windows10Compatible".to_string(),
                    windows10_compatible.to_string(),
                ].join(",")
            }),


            self.a_wcc_interface.as_ref().map(|a_wcc_interface| {
                vec![
                    "aWCCInterface".to_string(),
                    a_wcc_interface.to_string(),
                ].join(",")
            }),


            self.multi_monitor_support.as_ref().map(|multi_monitor_support| {
                vec![
                    "multiMonitorSupport".to_string(),
                    multi_monitor_support.to_string(),
                ].join(",")
            }),


            self.heat_management_system.as_ref().map(|heat_management_system| {
                vec![
                    "heatManagementSystem".to_string(),
                    heat_management_system.to_string(),
                ].join(",")
            }),


            self.param_1_tbss_d_storage.as_ref().map(|param_1_tbss_d_storage| {
                vec![
                    "1TBSsDStorage".to_string(),
                    param_1_tbss_d_storage.to_string(),
                ].join(",")
            }),


            self.param_8th_gen_processor.as_ref().map(|param_8th_gen_processor| {
                vec![
                    "8thGenProcessor".to_string(),
                    param_8th_gen_processor.to_string(),
                ].join(",")
            }),


            self.high_definition_audio_out.as_ref().map(|high_definition_audio_out| {
                vec![
                    "highDefinitionAudioOut".to_string(),
                    high_definition_audio_out.to_string(),
                ].join(",")
            }),


            Some("gPO-fb8".to_string()),
            Some(self.g_po_fb8.to_string()),


            Some("gPO-e23".to_string()),
            Some(self.g_po_e23.to_string()),


            self.liquid_cooling_option.as_ref().map(|liquid_cooling_option| {
                vec![
                    "liquidCoolingOption".to_string(),
                    liquid_cooling_option.to_string(),
                ].join(",")
            }),


            self.built_in_wi_fi.as_ref().map(|built_in_wi_fi| {
                vec![
                    "built-inWi-Fi".to_string(),
                    built_in_wi_fi.to_string(),
                ].join(",")
            }),


            self.customizable_lighting.as_ref().map(|customizable_lighting| {
                vec![
                    "customizableLighting".to_string(),
                    customizable_lighting.to_string(),
                ].join(",")
            }),


            self.max_ra_m_capacity.as_ref().map(|max_ra_m_capacity| {
                vec![
                    "maxRaMCapacity".to_string(),
                    max_ra_m_capacity.to_string(),
                ].join(",")
            }),


            self.param_7_us_b_ports.as_ref().map(|param_7_us_b_ports| {
                vec![
                    "7UsBPorts".to_string(),
                    param_7_us_b_ports.to_string(),
                ].join(",")
            }),


            self.smart_airflow_technology.as_ref().map(|smart_airflow_technology| {
                vec![
                    "smartAirflowTechnology".to_string(),
                    smart_airflow_technology.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AlienwareAuroraR15GamingDesktop value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AlienwareAuroraR15GamingDesktop {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub n_vidia_graphics: Vec<String>,
            pub h_dmi_outputs: Vec<i64>,
            pub windows10_compatible: Vec<bool>,
            pub a_wcc_interface: Vec<String>,
            pub multi_monitor_support: Vec<bool>,
            pub heat_management_system: Vec<bool>,
            pub param_1_tbss_d_storage: Vec<f64>,
            pub param_8th_gen_processor: Vec<String>,
            pub high_definition_audio_out: Vec<bool>,
            pub g_po_fb8: Vec<i64>,
            pub g_po_e23: Vec<String>,
            pub liquid_cooling_option: Vec<bool>,
            pub built_in_wi_fi: Vec<bool>,
            pub customizable_lighting: Vec<bool>,
            pub max_ra_m_capacity: Vec<i64>,
            pub param_7_us_b_ports: Vec<i64>,
            pub smart_airflow_technology: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AlienwareAuroraR15GamingDesktop".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "nVidiaGraphics" => intermediate_rep.n_vidia_graphics.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hDMIOutputs" => intermediate_rep.h_dmi_outputs.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "windows10Compatible" => intermediate_rep.windows10_compatible.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "aWCCInterface" => intermediate_rep.a_wcc_interface.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "multiMonitorSupport" => intermediate_rep.multi_monitor_support.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "heatManagementSystem" => intermediate_rep.heat_management_system.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "1TBSsDStorage" => intermediate_rep.param_1_tbss_d_storage.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "8thGenProcessor" => intermediate_rep.param_8th_gen_processor.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "highDefinitionAudioOut" => intermediate_rep.high_definition_audio_out.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-fb8" => intermediate_rep.g_po_fb8.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-e23" => intermediate_rep.g_po_e23.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "liquidCoolingOption" => intermediate_rep.liquid_cooling_option.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "built-inWi-Fi" => intermediate_rep.built_in_wi_fi.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "customizableLighting" => intermediate_rep.customizable_lighting.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxRaMCapacity" => intermediate_rep.max_ra_m_capacity.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "7UsBPorts" => intermediate_rep.param_7_us_b_ports.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "smartAirflowTechnology" => intermediate_rep.smart_airflow_technology.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AlienwareAuroraR15GamingDesktop".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AlienwareAuroraR15GamingDesktop {
            n_vidia_graphics: intermediate_rep.n_vidia_graphics.into_iter().next(),
            h_dmi_outputs: intermediate_rep.h_dmi_outputs.into_iter().next(),
            windows10_compatible: intermediate_rep.windows10_compatible.into_iter().next(),
            a_wcc_interface: intermediate_rep.a_wcc_interface.into_iter().next(),
            multi_monitor_support: intermediate_rep.multi_monitor_support.into_iter().next(),
            heat_management_system: intermediate_rep.heat_management_system.into_iter().next(),
            param_1_tbss_d_storage: intermediate_rep.param_1_tbss_d_storage.into_iter().next(),
            param_8th_gen_processor: intermediate_rep.param_8th_gen_processor.into_iter().next(),
            high_definition_audio_out: intermediate_rep.high_definition_audio_out.into_iter().next(),
            g_po_fb8: intermediate_rep.g_po_fb8.into_iter().next().ok_or_else(|| "gPO-fb8 missing in AlienwareAuroraR15GamingDesktop".to_string())?,
            g_po_e23: intermediate_rep.g_po_e23.into_iter().next().ok_or_else(|| "gPO-e23 missing in AlienwareAuroraR15GamingDesktop".to_string())?,
            liquid_cooling_option: intermediate_rep.liquid_cooling_option.into_iter().next(),
            built_in_wi_fi: intermediate_rep.built_in_wi_fi.into_iter().next(),
            customizable_lighting: intermediate_rep.customizable_lighting.into_iter().next(),
            max_ra_m_capacity: intermediate_rep.max_ra_m_capacity.into_iter().next(),
            param_7_us_b_ports: intermediate_rep.param_7_us_b_ports.into_iter().next(),
            smart_airflow_technology: intermediate_rep.smart_airflow_technology.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AlienwareAuroraR15GamingDesktop> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AlienwareAuroraR15GamingDesktop>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AlienwareAuroraR15GamingDesktop>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AlienwareAuroraR15GamingDesktop - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AlienwareAuroraR15GamingDesktop> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AlienwareAuroraR15GamingDesktop as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AlienwareAuroraR15GamingDesktop - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl AlienwareAuroraR15GamingDesktop {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn as_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "cat")]
pub struct Cat {
    /// This attribute is a variable named:sleeps hours
    #[serde(rename = "sleepsHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sleeps_hours: Option<i64>,

    /// This attribute is a variable named:eye color
    #[serde(rename = "eyeColor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub eye_color: Option<String>,

    /// This attribute is a variable named:furry tail
    #[serde(rename = "furryTail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub furry_tail: Option<bool>,

    /// This attribute is a variable named:age
    #[serde(rename = "age")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub age: Option<i64>,

    /// This attribute is a variable named:jump height
    #[serde(rename = "jumpHeight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jump_height: Option<f64>,

    /// This attribute is a variable named:perches places
    #[serde(rename = "perchesPlaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub perches_places: Option<bool>,

    /// This attribute is a variable named:moody temperament
    #[serde(rename = "moodyTemperament")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub moody_temperament: Option<bool>,

    /// This attribute is a variable named:tail length
    #[serde(rename = "tailLength")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tail_length: Option<i64>,

    /// This attribute is a variable named:coat pattern
    #[serde(rename = "coatPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub coat_pattern: Option<String>,

    /// This attribute is a variable named:likes people
    #[serde(rename = "likesPeople")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub likes_people: Option<bool>,

    /// This attribute is a variable named:meow frequency
    #[serde(rename = "meowFrequency")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meow_frequency: Option<i64>,

    /// This attribute is a variable named:fur color
    #[serde(rename = "furColor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fur_color: Option<String>,

    #[serde(rename = "gPO-449")]
    pub g_po_449: i64,

    /// This attribute is a variable named:weight
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub weight: Option<f64>,

    #[serde(rename = "gPO-256")]
    pub g_po_256: String,

    /// This attribute is a variable named:has claws
    #[serde(rename = "hasClaws")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_claws: Option<bool>,

    /// This attribute is a variable named:breed
    #[serde(rename = "breed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub breed: Option<String>,

}

impl Cat {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Cat {
        Cat {
            sleeps_hours: Some(0),
            eye_color: None,
            furry_tail: Some(false),
            age: Some(0),
            jump_height: None,
            perches_places: Some(false),
            moody_temperament: Some(false),
            tail_length: Some(0),
            coat_pattern: None,
            likes_people: Some(false),
            meow_frequency: Some(0),
            fur_color: None,
            g_po_449: 3096,
            weight: Some(0.0),
            g_po_256: "3097".to_string(),
            has_claws: Some(false),
            breed: None,
        }
    }
}

/// Converts the Cat value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Cat {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.sleeps_hours.as_ref().map(|sleeps_hours| {
                vec![
                    "sleepsHours".to_string(),
                    sleeps_hours.to_string(),
                ].join(",")
            }),


            self.eye_color.as_ref().map(|eye_color| {
                vec![
                    "eyeColor".to_string(),
                    eye_color.to_string(),
                ].join(",")
            }),


            self.furry_tail.as_ref().map(|furry_tail| {
                vec![
                    "furryTail".to_string(),
                    furry_tail.to_string(),
                ].join(",")
            }),


            self.age.as_ref().map(|age| {
                vec![
                    "age".to_string(),
                    age.to_string(),
                ].join(",")
            }),


            self.jump_height.as_ref().map(|jump_height| {
                vec![
                    "jumpHeight".to_string(),
                    jump_height.to_string(),
                ].join(",")
            }),


            self.perches_places.as_ref().map(|perches_places| {
                vec![
                    "perchesPlaces".to_string(),
                    perches_places.to_string(),
                ].join(",")
            }),


            self.moody_temperament.as_ref().map(|moody_temperament| {
                vec![
                    "moodyTemperament".to_string(),
                    moody_temperament.to_string(),
                ].join(",")
            }),


            self.tail_length.as_ref().map(|tail_length| {
                vec![
                    "tailLength".to_string(),
                    tail_length.to_string(),
                ].join(",")
            }),


            self.coat_pattern.as_ref().map(|coat_pattern| {
                vec![
                    "coatPattern".to_string(),
                    coat_pattern.to_string(),
                ].join(",")
            }),


            self.likes_people.as_ref().map(|likes_people| {
                vec![
                    "likesPeople".to_string(),
                    likes_people.to_string(),
                ].join(",")
            }),


            self.meow_frequency.as_ref().map(|meow_frequency| {
                vec![
                    "meowFrequency".to_string(),
                    meow_frequency.to_string(),
                ].join(",")
            }),


            self.fur_color.as_ref().map(|fur_color| {
                vec![
                    "furColor".to_string(),
                    fur_color.to_string(),
                ].join(",")
            }),


            Some("gPO-449".to_string()),
            Some(self.g_po_449.to_string()),


            self.weight.as_ref().map(|weight| {
                vec![
                    "weight".to_string(),
                    weight.to_string(),
                ].join(",")
            }),


            Some("gPO-256".to_string()),
            Some(self.g_po_256.to_string()),


            self.has_claws.as_ref().map(|has_claws| {
                vec![
                    "hasClaws".to_string(),
                    has_claws.to_string(),
                ].join(",")
            }),


            self.breed.as_ref().map(|breed| {
                vec![
                    "breed".to_string(),
                    breed.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Cat value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Cat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub sleeps_hours: Vec<i64>,
            pub eye_color: Vec<String>,
            pub furry_tail: Vec<bool>,
            pub age: Vec<i64>,
            pub jump_height: Vec<f64>,
            pub perches_places: Vec<bool>,
            pub moody_temperament: Vec<bool>,
            pub tail_length: Vec<i64>,
            pub coat_pattern: Vec<String>,
            pub likes_people: Vec<bool>,
            pub meow_frequency: Vec<i64>,
            pub fur_color: Vec<String>,
            pub g_po_449: Vec<i64>,
            pub weight: Vec<f64>,
            pub g_po_256: Vec<String>,
            pub has_claws: Vec<bool>,
            pub breed: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Cat".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "sleepsHours" => intermediate_rep.sleeps_hours.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "eyeColor" => intermediate_rep.eye_color.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "furryTail" => intermediate_rep.furry_tail.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "age" => intermediate_rep.age.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "jumpHeight" => intermediate_rep.jump_height.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "perchesPlaces" => intermediate_rep.perches_places.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "moodyTemperament" => intermediate_rep.moody_temperament.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tailLength" => intermediate_rep.tail_length.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "coatPattern" => intermediate_rep.coat_pattern.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "likesPeople" => intermediate_rep.likes_people.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "meowFrequency" => intermediate_rep.meow_frequency.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "furColor" => intermediate_rep.fur_color.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-449" => intermediate_rep.g_po_449.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "weight" => intermediate_rep.weight.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-256" => intermediate_rep.g_po_256.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hasClaws" => intermediate_rep.has_claws.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "breed" => intermediate_rep.breed.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Cat".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Cat {
            sleeps_hours: intermediate_rep.sleeps_hours.into_iter().next(),
            eye_color: intermediate_rep.eye_color.into_iter().next(),
            furry_tail: intermediate_rep.furry_tail.into_iter().next(),
            age: intermediate_rep.age.into_iter().next(),
            jump_height: intermediate_rep.jump_height.into_iter().next(),
            perches_places: intermediate_rep.perches_places.into_iter().next(),
            moody_temperament: intermediate_rep.moody_temperament.into_iter().next(),
            tail_length: intermediate_rep.tail_length.into_iter().next(),
            coat_pattern: intermediate_rep.coat_pattern.into_iter().next(),
            likes_people: intermediate_rep.likes_people.into_iter().next(),
            meow_frequency: intermediate_rep.meow_frequency.into_iter().next(),
            fur_color: intermediate_rep.fur_color.into_iter().next(),
            g_po_449: intermediate_rep.g_po_449.into_iter().next().ok_or_else(|| "gPO-449 missing in Cat".to_string())?,
            weight: intermediate_rep.weight.into_iter().next(),
            g_po_256: intermediate_rep.g_po_256.into_iter().next().ok_or_else(|| "gPO-256 missing in Cat".to_string())?,
            has_claws: intermediate_rep.has_claws.into_iter().next(),
            breed: intermediate_rep.breed.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Cat> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Cat>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Cat>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Cat - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Cat> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Cat as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Cat - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl Cat {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn as_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "dell274kUhDMonitor-S2721QS")]
pub struct Dell274kUhdMonitorS2721qs {
    /// This attribute is a variable named:Integrated Speakers
    #[serde(rename = "integratedSpeakers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub integrated_speakers: Option<bool>,

    /// This attribute is a variable named:Color Depth
    #[serde(rename = "colorDepth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color_depth: Option<i64>,

    /// This attribute is a variable named:Response Time
    #[serde(rename = "responseTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub response_time: Option<i64>,

    /// This attribute is a variable named:Resolution
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolution: Option<i64>,

    /// This attribute is a variable named:Viewing Angle
    #[serde(rename = "viewingAngle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub viewing_angle: Option<i64>,

    /// This attribute is a variable named:Tiltable Stand
    #[serde(rename = "tiltableStand")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tiltable_stand: Option<bool>,

    /// This attribute is a variable named:Price Range
    #[serde(rename = "priceRange")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub price_range: Option<f64>,

    /// This attribute is a variable named:Screen Size
    #[serde(rename = "screenSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub screen_size: Option<i64>,

    /// This attribute is a variable named:Brightness
    #[serde(rename = "brightness")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub brightness: Option<i64>,

    /// This attribute is a variable named:Color Gamut
    #[serde(rename = "colorGamut")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color_gamut: Option<String>,

    #[serde(rename = "gPO-428")]
    pub g_po_428: i64,

    /// This attribute is a variable named:Panel Type
    #[serde(rename = "panelType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub panel_type: Option<String>,

    /// This attribute is a variable named:VESA Mountable
    #[serde(rename = "vESAMountable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub v_esa_mountable: Option<bool>,

    #[serde(rename = "gPO-cf2")]
    pub g_po_cf2: String,

    /// This attribute is a variable named:Connectivity Ports
    #[serde(rename = "connectivityPorts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connectivity_ports: Option<String>,

    /// This attribute is a variable named:Contrast Ratio
    #[serde(rename = "contrastRatio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contrast_ratio: Option<f64>,

    /// This attribute is a variable named:Aspect Ratio
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aspect_ratio: Option<f64>,

}

impl Dell274kUhdMonitorS2721qs {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Dell274kUhdMonitorS2721qs {
        Dell274kUhdMonitorS2721qs {
            integrated_speakers: Some(false),
            color_depth: Some(0),
            response_time: Some(0),
            resolution: Some(0),
            viewing_angle: Some(0),
            tiltable_stand: Some(false),
            price_range: None,
            screen_size: Some(0),
            brightness: Some(0),
            color_gamut: None,
            g_po_428: 4316,
            panel_type: None,
            v_esa_mountable: Some(false),
            g_po_cf2: "4317".to_string(),
            connectivity_ports: None,
            contrast_ratio: None,
            aspect_ratio: Some(0.0),
        }
    }
}

/// Converts the Dell274kUhdMonitorS2721qs value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Dell274kUhdMonitorS2721qs {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.integrated_speakers.as_ref().map(|integrated_speakers| {
                vec![
                    "integratedSpeakers".to_string(),
                    integrated_speakers.to_string(),
                ].join(",")
            }),


            self.color_depth.as_ref().map(|color_depth| {
                vec![
                    "colorDepth".to_string(),
                    color_depth.to_string(),
                ].join(",")
            }),


            self.response_time.as_ref().map(|response_time| {
                vec![
                    "responseTime".to_string(),
                    response_time.to_string(),
                ].join(",")
            }),


            self.resolution.as_ref().map(|resolution| {
                vec![
                    "resolution".to_string(),
                    resolution.to_string(),
                ].join(",")
            }),


            self.viewing_angle.as_ref().map(|viewing_angle| {
                vec![
                    "viewingAngle".to_string(),
                    viewing_angle.to_string(),
                ].join(",")
            }),


            self.tiltable_stand.as_ref().map(|tiltable_stand| {
                vec![
                    "tiltableStand".to_string(),
                    tiltable_stand.to_string(),
                ].join(",")
            }),


            self.price_range.as_ref().map(|price_range| {
                vec![
                    "priceRange".to_string(),
                    price_range.to_string(),
                ].join(",")
            }),


            self.screen_size.as_ref().map(|screen_size| {
                vec![
                    "screenSize".to_string(),
                    screen_size.to_string(),
                ].join(",")
            }),


            self.brightness.as_ref().map(|brightness| {
                vec![
                    "brightness".to_string(),
                    brightness.to_string(),
                ].join(",")
            }),


            self.color_gamut.as_ref().map(|color_gamut| {
                vec![
                    "colorGamut".to_string(),
                    color_gamut.to_string(),
                ].join(",")
            }),


            Some("gPO-428".to_string()),
            Some(self.g_po_428.to_string()),


            self.panel_type.as_ref().map(|panel_type| {
                vec![
                    "panelType".to_string(),
                    panel_type.to_string(),
                ].join(",")
            }),


            self.v_esa_mountable.as_ref().map(|v_esa_mountable| {
                vec![
                    "vESAMountable".to_string(),
                    v_esa_mountable.to_string(),
                ].join(",")
            }),


            Some("gPO-cf2".to_string()),
            Some(self.g_po_cf2.to_string()),


            self.connectivity_ports.as_ref().map(|connectivity_ports| {
                vec![
                    "connectivityPorts".to_string(),
                    connectivity_ports.to_string(),
                ].join(",")
            }),


            self.contrast_ratio.as_ref().map(|contrast_ratio| {
                vec![
                    "contrastRatio".to_string(),
                    contrast_ratio.to_string(),
                ].join(",")
            }),


            self.aspect_ratio.as_ref().map(|aspect_ratio| {
                vec![
                    "aspectRatio".to_string(),
                    aspect_ratio.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Dell274kUhdMonitorS2721qs value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Dell274kUhdMonitorS2721qs {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub integrated_speakers: Vec<bool>,
            pub color_depth: Vec<i64>,
            pub response_time: Vec<i64>,
            pub resolution: Vec<i64>,
            pub viewing_angle: Vec<i64>,
            pub tiltable_stand: Vec<bool>,
            pub price_range: Vec<f64>,
            pub screen_size: Vec<i64>,
            pub brightness: Vec<i64>,
            pub color_gamut: Vec<String>,
            pub g_po_428: Vec<i64>,
            pub panel_type: Vec<String>,
            pub v_esa_mountable: Vec<bool>,
            pub g_po_cf2: Vec<String>,
            pub connectivity_ports: Vec<String>,
            pub contrast_ratio: Vec<f64>,
            pub aspect_ratio: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Dell274kUhdMonitorS2721qs".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "integratedSpeakers" => intermediate_rep.integrated_speakers.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "colorDepth" => intermediate_rep.color_depth.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "responseTime" => intermediate_rep.response_time.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "resolution" => intermediate_rep.resolution.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "viewingAngle" => intermediate_rep.viewing_angle.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tiltableStand" => intermediate_rep.tiltable_stand.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "priceRange" => intermediate_rep.price_range.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "screenSize" => intermediate_rep.screen_size.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "brightness" => intermediate_rep.brightness.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "colorGamut" => intermediate_rep.color_gamut.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-428" => intermediate_rep.g_po_428.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "panelType" => intermediate_rep.panel_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vESAMountable" => intermediate_rep.v_esa_mountable.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-cf2" => intermediate_rep.g_po_cf2.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "connectivityPorts" => intermediate_rep.connectivity_ports.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contrastRatio" => intermediate_rep.contrast_ratio.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "aspectRatio" => intermediate_rep.aspect_ratio.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Dell274kUhdMonitorS2721qs".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Dell274kUhdMonitorS2721qs {
            integrated_speakers: intermediate_rep.integrated_speakers.into_iter().next(),
            color_depth: intermediate_rep.color_depth.into_iter().next(),
            response_time: intermediate_rep.response_time.into_iter().next(),
            resolution: intermediate_rep.resolution.into_iter().next(),
            viewing_angle: intermediate_rep.viewing_angle.into_iter().next(),
            tiltable_stand: intermediate_rep.tiltable_stand.into_iter().next(),
            price_range: intermediate_rep.price_range.into_iter().next(),
            screen_size: intermediate_rep.screen_size.into_iter().next(),
            brightness: intermediate_rep.brightness.into_iter().next(),
            color_gamut: intermediate_rep.color_gamut.into_iter().next(),
            g_po_428: intermediate_rep.g_po_428.into_iter().next().ok_or_else(|| "gPO-428 missing in Dell274kUhdMonitorS2721qs".to_string())?,
            panel_type: intermediate_rep.panel_type.into_iter().next(),
            v_esa_mountable: intermediate_rep.v_esa_mountable.into_iter().next(),
            g_po_cf2: intermediate_rep.g_po_cf2.into_iter().next().ok_or_else(|| "gPO-cf2 missing in Dell274kUhdMonitorS2721qs".to_string())?,
            connectivity_ports: intermediate_rep.connectivity_ports.into_iter().next(),
            contrast_ratio: intermediate_rep.contrast_ratio.into_iter().next(),
            aspect_ratio: intermediate_rep.aspect_ratio.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Dell274kUhdMonitorS2721qs> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Dell274kUhdMonitorS2721qs>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Dell274kUhdMonitorS2721qs>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Dell274kUhdMonitorS2721qs - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Dell274kUhdMonitorS2721qs> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Dell274kUhdMonitorS2721qs as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Dell274kUhdMonitorS2721qs - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl Dell274kUhdMonitorS2721qs {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn as_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "dog")]
pub struct Dog {
    /// This attribute is a variable named:Affectionate
    #[serde(rename = "affectionate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub affectionate: Option<bool>,

    /// This attribute is a variable named:Loyal
    #[serde(rename = "loyal")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub loyal: Option<bool>,

    /// This attribute is a variable named:Breed
    #[serde(rename = "breed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub breed: Option<String>,

    /// This attribute is a variable named:Weight
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub weight: Option<f64>,

    /// This attribute is a variable named:Height
    #[serde(rename = "height")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub height: Option<f64>,

    /// This attribute is a variable named:Noisy
    #[serde(rename = "noisy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub noisy: Option<bool>,

    /// This attribute is a variable named:Energetic
    #[serde(rename = "energetic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub energetic: Option<bool>,

    /// This attribute is a variable named:Friendly
    #[serde(rename = "friendly")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub friendly: Option<bool>,

    /// This attribute is a variable named:Healthy
    #[serde(rename = "healthy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub healthy: Option<bool>,

    /// This attribute is a variable named:Vaccinated
    #[serde(rename = "vaccinated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vaccinated: Option<bool>,

    #[serde(rename = "gPO-df0")]
    pub g_po_df0: i64,

    /// This attribute is a variable named:Age
    #[serde(rename = "age")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub age: Option<i64>,

    /// This attribute is a variable named:Intelligent
    #[serde(rename = "intelligent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub intelligent: Option<bool>,

    #[serde(rename = "gPO-e4a")]
    pub g_po_e4a: String,

    /// This attribute is a variable named:Gender
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gender: Option<String>,

    /// This attribute is a variable named:Colour
    #[serde(rename = "colour")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub colour: Option<String>,

}

impl Dog {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Dog {
        Dog {
            affectionate: Some(false),
            loyal: Some(false),
            breed: None,
            weight: None,
            height: None,
            noisy: Some(false),
            energetic: Some(false),
            friendly: Some(false),
            healthy: Some(false),
            vaccinated: Some(false),
            g_po_df0: 4318,
            age: Some(0),
            intelligent: Some(false),
            g_po_e4a: "4319".to_string(),
            gender: None,
            colour: None,
        }
    }
}

/// Converts the Dog value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Dog {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.affectionate.as_ref().map(|affectionate| {
                vec![
                    "affectionate".to_string(),
                    affectionate.to_string(),
                ].join(",")
            }),


            self.loyal.as_ref().map(|loyal| {
                vec![
                    "loyal".to_string(),
                    loyal.to_string(),
                ].join(",")
            }),


            self.breed.as_ref().map(|breed| {
                vec![
                    "breed".to_string(),
                    breed.to_string(),
                ].join(",")
            }),


            self.weight.as_ref().map(|weight| {
                vec![
                    "weight".to_string(),
                    weight.to_string(),
                ].join(",")
            }),


            self.height.as_ref().map(|height| {
                vec![
                    "height".to_string(),
                    height.to_string(),
                ].join(",")
            }),


            self.noisy.as_ref().map(|noisy| {
                vec![
                    "noisy".to_string(),
                    noisy.to_string(),
                ].join(",")
            }),


            self.energetic.as_ref().map(|energetic| {
                vec![
                    "energetic".to_string(),
                    energetic.to_string(),
                ].join(",")
            }),


            self.friendly.as_ref().map(|friendly| {
                vec![
                    "friendly".to_string(),
                    friendly.to_string(),
                ].join(",")
            }),


            self.healthy.as_ref().map(|healthy| {
                vec![
                    "healthy".to_string(),
                    healthy.to_string(),
                ].join(",")
            }),


            self.vaccinated.as_ref().map(|vaccinated| {
                vec![
                    "vaccinated".to_string(),
                    vaccinated.to_string(),
                ].join(",")
            }),


            Some("gPO-df0".to_string()),
            Some(self.g_po_df0.to_string()),


            self.age.as_ref().map(|age| {
                vec![
                    "age".to_string(),
                    age.to_string(),
                ].join(",")
            }),


            self.intelligent.as_ref().map(|intelligent| {
                vec![
                    "intelligent".to_string(),
                    intelligent.to_string(),
                ].join(",")
            }),


            Some("gPO-e4a".to_string()),
            Some(self.g_po_e4a.to_string()),


            self.gender.as_ref().map(|gender| {
                vec![
                    "gender".to_string(),
                    gender.to_string(),
                ].join(",")
            }),


            self.colour.as_ref().map(|colour| {
                vec![
                    "colour".to_string(),
                    colour.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Dog value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Dog {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub affectionate: Vec<bool>,
            pub loyal: Vec<bool>,
            pub breed: Vec<String>,
            pub weight: Vec<f64>,
            pub height: Vec<f64>,
            pub noisy: Vec<bool>,
            pub energetic: Vec<bool>,
            pub friendly: Vec<bool>,
            pub healthy: Vec<bool>,
            pub vaccinated: Vec<bool>,
            pub g_po_df0: Vec<i64>,
            pub age: Vec<i64>,
            pub intelligent: Vec<bool>,
            pub g_po_e4a: Vec<String>,
            pub gender: Vec<String>,
            pub colour: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Dog".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "affectionate" => intermediate_rep.affectionate.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "loyal" => intermediate_rep.loyal.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "breed" => intermediate_rep.breed.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "weight" => intermediate_rep.weight.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "height" => intermediate_rep.height.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "noisy" => intermediate_rep.noisy.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "energetic" => intermediate_rep.energetic.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "friendly" => intermediate_rep.friendly.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "healthy" => intermediate_rep.healthy.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vaccinated" => intermediate_rep.vaccinated.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-df0" => intermediate_rep.g_po_df0.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "age" => intermediate_rep.age.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "intelligent" => intermediate_rep.intelligent.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-e4a" => intermediate_rep.g_po_e4a.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gender" => intermediate_rep.gender.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "colour" => intermediate_rep.colour.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Dog".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Dog {
            affectionate: intermediate_rep.affectionate.into_iter().next(),
            loyal: intermediate_rep.loyal.into_iter().next(),
            breed: intermediate_rep.breed.into_iter().next(),
            weight: intermediate_rep.weight.into_iter().next(),
            height: intermediate_rep.height.into_iter().next(),
            noisy: intermediate_rep.noisy.into_iter().next(),
            energetic: intermediate_rep.energetic.into_iter().next(),
            friendly: intermediate_rep.friendly.into_iter().next(),
            healthy: intermediate_rep.healthy.into_iter().next(),
            vaccinated: intermediate_rep.vaccinated.into_iter().next(),
            g_po_df0: intermediate_rep.g_po_df0.into_iter().next().ok_or_else(|| "gPO-df0 missing in Dog".to_string())?,
            age: intermediate_rep.age.into_iter().next(),
            intelligent: intermediate_rep.intelligent.into_iter().next(),
            g_po_e4a: intermediate_rep.g_po_e4a.into_iter().next().ok_or_else(|| "gPO-e4a missing in Dog".to_string())?,
            gender: intermediate_rep.gender.into_iter().next(),
            colour: intermediate_rep.colour.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Dog> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Dog>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Dog>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Dog - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Dog> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Dog as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Dog - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl Dog {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn as_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "error")]
pub struct Error {
    #[serde(rename = "code")]
    pub code: i32,

    #[serde(rename = "message")]
    pub message: String,

}

impl Error {
    #[allow(clippy::new_without_default)]
    pub fn new(code: i32, message: String, ) -> Error {
        Error {
            code,
            message,
        }
    }
}

/// Converts the Error value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("code".to_string()),
            Some(self.code.to_string()),


            Some("message".to_string()),
            Some(self.message.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Error value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Error {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Error".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Error".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Error {
            code: intermediate_rep.code.into_iter().next().ok_or_else(|| "code missing in Error".to_string())?,
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in Error".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Error> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Error>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Error>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Error - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Error> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Error as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Error - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl Error {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn as_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor")]
pub struct LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
    /// This attribute is a variable named:PortsNumber
    #[serde(rename = "portsNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ports_number: Option<i64>,

    /// This attribute is a variable named:CardReaderType
    #[serde(rename = "cardReaderType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub card_reader_type: Option<String>,

    /// This attribute is a variable named:Generation
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub generation: Option<i64>,

    /// This attribute is a variable named:Processor
    #[serde(rename = "processor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processor: Option<String>,

    /// This attribute is a variable named:Storage
    #[serde(rename = "storage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage: Option<i64>,

    /// This attribute is a variable named:Memory
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory: Option<i64>,

    /// This attribute is a variable named:Speed
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub speed: Option<f64>,

    /// This attribute is a variable named:PowerSupplyUnitSizeWatts
    #[serde(rename = "powerSupplyUnitSizeWatts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub power_supply_unit_size_watts: Option<i64>,

    #[serde(rename = "gPO-a87")]
    pub g_po_a87: i64,

    /// This attribute is a variable named:ColorSchemeOption1and2
    #[serde(rename = "colorSchemeOption1and2")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color_scheme_option1and2: Option<String>,

    /// This attribute is a variable named:Cores
    #[serde(rename = "cores")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cores: Option<i64>,

    /// This attribute is a variable named:Graphics
    #[serde(rename = "graphics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub graphics: Option<String>,

    #[serde(rename = "gPO-181")]
    pub g_po_181: String,

    /// This attribute is a variable named:OperatingSystem
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_system: Option<String>,

    /// This attribute is a variable named:Model
    #[serde(rename = "model")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub model: Option<String>,

    /// This attribute is a variable named:Brand
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub brand: Option<String>,

    /// This attribute is a variable named:OpticalDriveType
    #[serde(rename = "opticalDriveType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub optical_drive_type: Option<String>,

}

impl LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
        LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
            ports_number: Some(0),
            card_reader_type: None,
            generation: Some(0),
            processor: None,
            storage: Some(0),
            memory: Some(0),
            speed: None,
            power_supply_unit_size_watts: Some(0),
            g_po_a87: 2628,
            color_scheme_option1and2: None,
            cores: Some(0),
            graphics: None,
            g_po_181: "2629".to_string(),
            operating_system: None,
            model: None,
            brand: None,
            optical_drive_type: None,
        }
    }
}

/// Converts the LenovoLegionTower7iGen8DesktopIntelCoreI9Processor value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.ports_number.as_ref().map(|ports_number| {
                vec![
                    "portsNumber".to_string(),
                    ports_number.to_string(),
                ].join(",")
            }),


            self.card_reader_type.as_ref().map(|card_reader_type| {
                vec![
                    "cardReaderType".to_string(),
                    card_reader_type.to_string(),
                ].join(",")
            }),


            self.generation.as_ref().map(|generation| {
                vec![
                    "generation".to_string(),
                    generation.to_string(),
                ].join(",")
            }),


            self.processor.as_ref().map(|processor| {
                vec![
                    "processor".to_string(),
                    processor.to_string(),
                ].join(",")
            }),


            self.storage.as_ref().map(|storage| {
                vec![
                    "storage".to_string(),
                    storage.to_string(),
                ].join(",")
            }),


            self.memory.as_ref().map(|memory| {
                vec![
                    "memory".to_string(),
                    memory.to_string(),
                ].join(",")
            }),


            self.speed.as_ref().map(|speed| {
                vec![
                    "speed".to_string(),
                    speed.to_string(),
                ].join(",")
            }),


            self.power_supply_unit_size_watts.as_ref().map(|power_supply_unit_size_watts| {
                vec![
                    "powerSupplyUnitSizeWatts".to_string(),
                    power_supply_unit_size_watts.to_string(),
                ].join(",")
            }),


            Some("gPO-a87".to_string()),
            Some(self.g_po_a87.to_string()),


            self.color_scheme_option1and2.as_ref().map(|color_scheme_option1and2| {
                vec![
                    "colorSchemeOption1and2".to_string(),
                    color_scheme_option1and2.to_string(),
                ].join(",")
            }),


            self.cores.as_ref().map(|cores| {
                vec![
                    "cores".to_string(),
                    cores.to_string(),
                ].join(",")
            }),


            self.graphics.as_ref().map(|graphics| {
                vec![
                    "graphics".to_string(),
                    graphics.to_string(),
                ].join(",")
            }),


            Some("gPO-181".to_string()),
            Some(self.g_po_181.to_string()),


            self.operating_system.as_ref().map(|operating_system| {
                vec![
                    "operatingSystem".to_string(),
                    operating_system.to_string(),
                ].join(",")
            }),


            self.model.as_ref().map(|model| {
                vec![
                    "model".to_string(),
                    model.to_string(),
                ].join(",")
            }),


            self.brand.as_ref().map(|brand| {
                vec![
                    "brand".to_string(),
                    brand.to_string(),
                ].join(",")
            }),


            self.optical_drive_type.as_ref().map(|optical_drive_type| {
                vec![
                    "opticalDriveType".to_string(),
                    optical_drive_type.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LenovoLegionTower7iGen8DesktopIntelCoreI9Processor value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub ports_number: Vec<i64>,
            pub card_reader_type: Vec<String>,
            pub generation: Vec<i64>,
            pub processor: Vec<String>,
            pub storage: Vec<i64>,
            pub memory: Vec<i64>,
            pub speed: Vec<f64>,
            pub power_supply_unit_size_watts: Vec<i64>,
            pub g_po_a87: Vec<i64>,
            pub color_scheme_option1and2: Vec<String>,
            pub cores: Vec<i64>,
            pub graphics: Vec<String>,
            pub g_po_181: Vec<String>,
            pub operating_system: Vec<String>,
            pub model: Vec<String>,
            pub brand: Vec<String>,
            pub optical_drive_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LenovoLegionTower7iGen8DesktopIntelCoreI9Processor".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "portsNumber" => intermediate_rep.ports_number.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cardReaderType" => intermediate_rep.card_reader_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "generation" => intermediate_rep.generation.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "processor" => intermediate_rep.processor.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "storage" => intermediate_rep.storage.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "memory" => intermediate_rep.memory.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "speed" => intermediate_rep.speed.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "powerSupplyUnitSizeWatts" => intermediate_rep.power_supply_unit_size_watts.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-a87" => intermediate_rep.g_po_a87.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "colorSchemeOption1and2" => intermediate_rep.color_scheme_option1and2.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cores" => intermediate_rep.cores.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "graphics" => intermediate_rep.graphics.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "gPO-181" => intermediate_rep.g_po_181.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "operatingSystem" => intermediate_rep.operating_system.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "model" => intermediate_rep.model.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "brand" => intermediate_rep.brand.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "opticalDriveType" => intermediate_rep.optical_drive_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LenovoLegionTower7iGen8DesktopIntelCoreI9Processor".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
            ports_number: intermediate_rep.ports_number.into_iter().next(),
            card_reader_type: intermediate_rep.card_reader_type.into_iter().next(),
            generation: intermediate_rep.generation.into_iter().next(),
            processor: intermediate_rep.processor.into_iter().next(),
            storage: intermediate_rep.storage.into_iter().next(),
            memory: intermediate_rep.memory.into_iter().next(),
            speed: intermediate_rep.speed.into_iter().next(),
            power_supply_unit_size_watts: intermediate_rep.power_supply_unit_size_watts.into_iter().next(),
            g_po_a87: intermediate_rep.g_po_a87.into_iter().next().ok_or_else(|| "gPO-a87 missing in LenovoLegionTower7iGen8DesktopIntelCoreI9Processor".to_string())?,
            color_scheme_option1and2: intermediate_rep.color_scheme_option1and2.into_iter().next(),
            cores: intermediate_rep.cores.into_iter().next(),
            graphics: intermediate_rep.graphics.into_iter().next(),
            g_po_181: intermediate_rep.g_po_181.into_iter().next().ok_or_else(|| "gPO-181 missing in LenovoLegionTower7iGen8DesktopIntelCoreI9Processor".to_string())?,
            operating_system: intermediate_rep.operating_system.into_iter().next(),
            model: intermediate_rep.model.into_iter().next(),
            brand: intermediate_rep.brand.into_iter().next(),
            optical_drive_type: intermediate_rep.optical_drive_type.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LenovoLegionTower7iGen8DesktopIntelCoreI9Processor> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LenovoLegionTower7iGen8DesktopIntelCoreI9Processor - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LenovoLegionTower7iGen8DesktopIntelCoreI9Processor> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LenovoLegionTower7iGen8DesktopIntelCoreI9Processor as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LenovoLegionTower7iGen8DesktopIntelCoreI9Processor - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl LenovoLegionTower7iGen8DesktopIntelCoreI9Processor {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn as_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "user")]
pub struct User {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "phone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phone: Option<String>,

    /// User Status
    #[serde(rename = "userStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_status: Option<i32>,

}

impl User {
    #[allow(clippy::new_without_default)]
    pub fn new() -> User {
        User {
            id: None,
            username: None,
            first_name: None,
            last_name: None,
            email: None,
            password: None,
            phone: None,
            user_status: None,
        }
    }
}

/// Converts the User value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for User {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                vec![
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.username.as_ref().map(|username| {
                vec![
                    "username".to_string(),
                    username.to_string(),
                ].join(",")
            }),


            self.first_name.as_ref().map(|first_name| {
                vec![
                    "firstName".to_string(),
                    first_name.to_string(),
                ].join(",")
            }),


            self.last_name.as_ref().map(|last_name| {
                vec![
                    "lastName".to_string(),
                    last_name.to_string(),
                ].join(",")
            }),


            self.email.as_ref().map(|email| {
                vec![
                    "email".to_string(),
                    email.to_string(),
                ].join(",")
            }),


            self.password.as_ref().map(|password| {
                vec![
                    "password".to_string(),
                    password.to_string(),
                ].join(",")
            }),


            self.phone.as_ref().map(|phone| {
                vec![
                    "phone".to_string(),
                    phone.to_string(),
                ].join(",")
            }),


            self.user_status.as_ref().map(|user_status| {
                vec![
                    "userStatus".to_string(),
                    user_status.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a User value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for User {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub username: Vec<String>,
            pub first_name: Vec<String>,
            pub last_name: Vec<String>,
            pub email: Vec<String>,
            pub password: Vec<String>,
            pub phone: Vec<String>,
            pub user_status: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing User".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "firstName" => intermediate_rep.first_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastName" => intermediate_rep.last_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phone" => intermediate_rep.phone.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "userStatus" => intermediate_rep.user_status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing User".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(User {
            id: intermediate_rep.id.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
            first_name: intermediate_rep.first_name.into_iter().next(),
            last_name: intermediate_rep.last_name.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            password: intermediate_rep.password.into_iter().next(),
            phone: intermediate_rep.phone.into_iter().next(),
            user_status: intermediate_rep.user_status.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<User> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<User>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<User>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for User - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<User> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <User as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into User - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl User {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn as_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}
