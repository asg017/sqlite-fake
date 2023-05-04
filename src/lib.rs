use fake::Fake;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, define_scalar_function, Result};

use fake::faker::{
    address::en::*,
    //administrative::en::*,
    //automotive::en::*,
    barcode::en::*,
    color::en::*,
    company::en::*,
    creditcard::en::*,
    currency::en::*,
    filesystem::en::*,
    finance::en::*,
    http::en::*,
    internet::en::*,
    job::en::{Field, Position, Seniority, Title as JobTitle},
    //lorem::en::*,
    name::en::*,
    phone_number::en::*,
};

macro_rules! define_fake {
    ($name:ident, $item:expr) => {
        pub fn $name(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
            api::result_text(context, $item().fake::<String>())?;
            Ok(())
        }
    };
}

// address
define_fake!(fake_city_prefix, CityPrefix);
define_fake!(fake_city_suffix, CitySuffix);
define_fake!(fake_city_name, CityName);
define_fake!(fake_country_name, CountryName);
define_fake!(fake_country_code, CountryCode);
define_fake!(fake_street_suffix, StreetSuffix);
define_fake!(fake_street_name, StreetName);
define_fake!(fake_time_zone, TimeZone);
define_fake!(fake_state_name, StateName);
define_fake!(fake_state_abbr, StateAbbr);
define_fake!(fake_secondary_address_type, SecondaryAddressType);
define_fake!(fake_secondary_address, SecondaryAddress);
define_fake!(fake_zip_code, ZipCode);
define_fake!(fake_post_code, PostCode);
define_fake!(fake_building_number, BuildingNumber);
define_fake!(fake_latitude, Latitude);
define_fake!(fake_longitude, Longitude);

//barcode
define_fake!(fake_isbn, Isbn);
define_fake!(fake_isbn10, Isbn10);
define_fake!(fake_isbn13, Isbn13);

//color
define_fake!(fake_hex_color, HexColor);
define_fake!(fake_rgb_color, RgbColor);
define_fake!(fake_rgba_color, RgbaColor);
define_fake!(fake_hsl_color, HslColor);
define_fake!(fake_hsla_color, HslaColor);
define_fake!(fake_color, Color);

// credit card
define_fake!(fake_credit_card, CreditCardNumber);

//company
define_fake!(fake_company_suffix, CompanySuffix);
define_fake!(fake_company_name, CompanyName);
define_fake!(fake_buzzword, Buzzword);
define_fake!(fake_buzzword_middle, BuzzwordMiddle);
define_fake!(fake_buzzword_tail, BuzzwordTail);
define_fake!(fake_catch_phase, CatchPhase);
define_fake!(fake_bs_verb, BsVerb);
define_fake!(fake_bs_adj, BsAdj);
define_fake!(fake_bs_noun, BsNoun);
define_fake!(fake_bs, Bs);
define_fake!(fake_profession, Profession);
define_fake!(fake_industry, Industry);

//http
define_fake!(fake_rfc_status_code, RfcStatusCode);
define_fake!(fake_valid_status_code, ValidStatusCode);

//internet
define_fake!(fake_free_email_provider, FreeEmailProvider);
define_fake!(fake_domain_suffix, DomainSuffix);
define_fake!(fake_free_email, FreeEmail);
define_fake!(fake_safe_email, SafeEmail);
define_fake!(fake_username, Username);
//define_fake!(fake_password, Password); TODO
define_fake!(fake_i_pv4, IPv4);
define_fake!(fake_i_pv6, IPv6);
define_fake!(fake_ip, IP);
define_fake!(fake_mac_address, MACAddress);
define_fake!(fake_user_agent, UserAgent);

// job
define_fake!(fake_seniority, Seniority);
define_fake!(fake_field, Field);
define_fake!(fake_position, Position);
define_fake!(fake_job_title, JobTitle);

// lorem TODO

// name
define_fake!(fake_name, Name);
define_fake!(fake_first_name, FirstName);
define_fake!(fake_last_name, LastName);
define_fake!(fake_title, Title);
define_fake!(fake_suffix, Suffix);
define_fake!(fake_name_with_title, NameWithTitle);

// number TODO

// phone number
define_fake!(fake_phone_number, PhoneNumber);
define_fake!(fake_cell_number, CellNumber);

// file system
define_fake!(fake_file_path, FilePath);
define_fake!(fake_file_name, FileName);
define_fake!(fake_dir_path, DirPath);
define_fake!(fake_mimetype, MimeType);
define_fake!(fake_semver, Semver);
define_fake!(fake_semver_stable, SemverStable);
define_fake!(fake_semver_unstable, SemverUnstable);
define_fake!(fake_file_extension, FileExtension);

// currency
define_fake!(fake_currency_code, CurrencyCode);
define_fake!(fake_currency_name, CurrencyName);
define_fake!(fake_currency_symbol, CurrencySymbol);

// finance
define_fake!(fake_bic, Bic);

// adminstrative
//define_fake!(fake_health_insurance_code, HealthInsuranceCode); TODO

// automotive
// define_fake!(fake_licence_plate, LicencePlate); TODO

#[sqlite_entrypoint]
pub fn sqlite3_fake_init(db: *mut sqlite3) -> Result<()> {
    macro_rules! register_fake {
        ($name:ident) => {
            define_scalar_function(db, stringify!($name), 0, $name, FunctionFlags::UTF8)?;
        };
    }
    register_fake!(fake_city_prefix);
    register_fake!(fake_city_suffix);
    register_fake!(fake_city_name);
    register_fake!(fake_country_name);
    register_fake!(fake_country_code);
    register_fake!(fake_street_suffix);
    register_fake!(fake_street_name);
    register_fake!(fake_time_zone);
    register_fake!(fake_state_name);
    register_fake!(fake_state_abbr);
    register_fake!(fake_secondary_address_type);
    register_fake!(fake_secondary_address);
    register_fake!(fake_zip_code);
    register_fake!(fake_post_code);
    register_fake!(fake_building_number);
    register_fake!(fake_latitude);
    register_fake!(fake_longitude);
    register_fake!(fake_isbn);
    register_fake!(fake_isbn10);
    register_fake!(fake_isbn13);
    register_fake!(fake_hex_color);
    register_fake!(fake_rgb_color);
    register_fake!(fake_rgba_color);
    register_fake!(fake_hsl_color);
    register_fake!(fake_hsla_color);
    register_fake!(fake_color);
    register_fake!(fake_credit_card);
    register_fake!(fake_company_suffix);
    register_fake!(fake_company_name);
    register_fake!(fake_buzzword);
    register_fake!(fake_buzzword_middle);
    register_fake!(fake_buzzword_tail);
    register_fake!(fake_catch_phase);
    register_fake!(fake_bs_verb);
    register_fake!(fake_bs_adj);
    register_fake!(fake_bs_noun);
    register_fake!(fake_bs);
    register_fake!(fake_profession);
    register_fake!(fake_industry);
    register_fake!(fake_rfc_status_code);
    register_fake!(fake_valid_status_code);
    register_fake!(fake_free_email_provider);
    register_fake!(fake_domain_suffix);
    register_fake!(fake_free_email);
    register_fake!(fake_safe_email);
    register_fake!(fake_username);
    register_fake!(fake_i_pv4);
    register_fake!(fake_i_pv6);
    register_fake!(fake_ip);
    register_fake!(fake_mac_address);
    register_fake!(fake_user_agent);
    register_fake!(fake_seniority);
    register_fake!(fake_field);
    register_fake!(fake_position);
    register_fake!(fake_job_title);
    register_fake!(fake_name);
    register_fake!(fake_first_name);
    register_fake!(fake_last_name);
    register_fake!(fake_title);
    register_fake!(fake_suffix);
    register_fake!(fake_name_with_title);
    register_fake!(fake_phone_number);
    register_fake!(fake_cell_number);
    register_fake!(fake_file_path);
    register_fake!(fake_file_name);
    register_fake!(fake_dir_path);
    register_fake!(fake_mimetype);
    register_fake!(fake_semver);
    register_fake!(fake_semver_stable);
    register_fake!(fake_semver_unstable);
    register_fake!(fake_file_extension);
    register_fake!(fake_currency_code);
    register_fake!(fake_currency_name);
    register_fake!(fake_currency_symbol);
    register_fake!(fake_bic);
    Ok(())
}
