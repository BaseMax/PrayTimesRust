use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Utc};
use pray_times::{
    types::{
        AsrFactor, CalculationUnit, Degrees, HighLatsMethod, Location, MidnightMethod, Minutes,
        Parameters, PraytimesOutput,
    },
    Calculator,
};
use serde_json::Value;
#[derive(Debug, Clone)]
struct TestCase {
    params: Parameters,
    location: Location,
    date: NaiveDate,
    expected_output: PraytimesOutput,
}

fn json_to_test_case(json: &Value) -> TestCase {
    let inputs = json["inputs"].as_object().unwrap();

    let params: Parameters = {
        let params_obj = inputs["params"].as_object().unwrap();

        let midnight = params_obj["midnight"].as_str().unwrap();
        let midnight = match midnight {
            "Jafari" => MidnightMethod::Jafari,
            "Standard" => MidnightMethod::Standard,
            _ => panic!("Invalid midnight method"),
        };
        let fajr = params_obj["fajr"].as_object().unwrap();
        let fajr_degree = fajr["degree"].as_f64().unwrap();
        let fajr = Degrees(fajr_degree);

        let dhuhr = params_obj["dhuhr"].as_object().unwrap();
        let dhuhr_minutes = dhuhr["minutes"].as_f64().unwrap();
        let dhuhr = Minutes(dhuhr_minutes);

        let asr = params_obj["asr"].as_object().unwrap();
        let asr_factor = asr["factor"].as_f64().unwrap();
        let asr = AsrFactor(asr_factor);

        let high_latitudes = match params_obj["highLats"].as_str().unwrap() {
            "None" => HighLatsMethod::None,
            "NightMiddle" => HighLatsMethod::NightMiddle,
            "OneSeventh" => HighLatsMethod::OneSeventh,
            "AngleBased" => HighLatsMethod::AngleBased,
            _ => panic!("Invalid high lat method"),
        };

        let maghrib = params_obj["maghrib"].as_object().unwrap();
        let maghrib = if maghrib.contains_key("minutes") {
            let maghrib_degree = maghrib["minutes"].as_f64().unwrap();
            CalculationUnit::Minutes(Minutes(maghrib_degree))
        } else {
            let maghrib_degree = maghrib["degree"].as_f64().unwrap();
            CalculationUnit::Degrees(Degrees(maghrib_degree))
        };

        let imsak = params_obj["imsak"].as_object().unwrap();
        let imsak = if imsak.contains_key("minutes") {
            let imsak_degree = imsak["minutes"].as_f64().unwrap();
            CalculationUnit::Minutes(Minutes(imsak_degree))
        } else {
            let imsak_degree = imsak["degree"].as_f64().unwrap();
            CalculationUnit::Degrees(Degrees(imsak_degree))
        };

        let isha = params_obj["isha"].as_object().unwrap();
        let isha = if isha.contains_key("minutes") {
            let isha_degree = isha["minutes"].as_f64().unwrap();
            CalculationUnit::Minutes(Minutes(isha_degree))
        } else {
            let isha_degree = isha["degree"].as_f64().unwrap();
            CalculationUnit::Degrees(Degrees(isha_degree))
        };

        Parameters {
            imsak,
            fajr,
            dhuhr,
            asr,
            maghrib,
            isha,
            midnight,
            high_latitudes,
        }
    };

    let location: Location = {
        let location_obj = inputs["location"].as_object().unwrap();

        let longitude = location_obj["longitude"].as_f64().unwrap();
        let latitude = location_obj["latitude"].as_f64().unwrap();
        let elevation = location_obj
            .get_key_value("elevation")
            .map(|(_, v)| v.as_f64().unwrap_or(0.0))
            .unwrap_or(0.0);

        Location {
            longitude,
            latitude,
            elevation,
        }
    };

    let date: NaiveDate = {
        let v: Vec<u32> = inputs["date"]
            .as_array()
            .unwrap()
            .iter()
            .map(|j| j.as_u64().unwrap() as u32)
            .collect::<Vec<_>>()
            .into();

        let d = NaiveDate::from_ymd_opt(v[0].try_into().unwrap(), v[1], v[2]).unwrap();

        d
    };

    let expected_output: PraytimesOutput = {
        let output_obj = json["originalOutput"].as_object().unwrap();
        PraytimesOutput {
            imsak: output_obj["imsak"].as_str().map(parse_date),
            fajr: output_obj["fajr"].as_str().map(parse_date),
            sunrise: output_obj["sunrise"].as_str().map(parse_date),
            dhuhr: output_obj["dhuhr"].as_str().map(parse_date),
            asr: output_obj["asr"].as_str().map(parse_date),
            sunset: output_obj["sunset"].as_str().map(parse_date),
            maghrib: output_obj["maghrib"].as_str().map(parse_date),
            isha: output_obj["isha"].as_str().map(parse_date),
            midnight: output_obj["midnight"].as_str().map(parse_date),
        }
    };

    TestCase {
        params,
        location,
        date: date.clone(),
        expected_output,
    }
}

fn parse_date(s: &str) -> NaiveDateTime {
    DateTime::parse_from_rfc3339(s).unwrap().naive_utc()
}

macro_rules! assert_datetime {
    ($real:expr, $expected:expr,$location:expr,$date:expr,$params:expr) => {
        {
            let real = $real;
            let expected = $expected;
            let location = $location;
            let date = $date;
            let params = $params;
            let res = match (&real, &expected) {
                (None, None) => true,
                (Some(real), Some(expected)) => {
                    (real.timestamp_millis() - expected.timestamp_millis()).abs() < 5000
                }
                _ => false,
            };


            assert!(
                res,
                "\n\ninvalid\t\t{real:?}\nexpected\t{expected:?} \n\n\nparameters : {params:?}\nlocation: {location:?}\ndate: {date:?}",
                real = real.unwrap(),
                expected = expected.unwrap()
            );
        }
    };
}

#[test]
fn should_match_with_the_main() {
    let data = std::fs::read_to_string("./assets/test-data.json").unwrap();
    let binding = serde_json::from_str::<Value>(&data).unwrap();
    let data = binding.as_array().unwrap();
    let data = data.iter().map(|v| json_to_test_case(v));

    // dbg!(&data);
    for TestCase {
        params,
        location,
        date,
        expected_output,
    } in data
    {
        dbg!(&date);
        let output: PraytimesOutput = Calculator::new(params.clone()).calculate(&location, &date);

        // assert_datetime!(
        //     output.imsak,
        //     expected_output.imsak,
        //     &location,
        //     date,
        //     &params
        // );
        // assert_datetime!(output.fajr, expected_output.fajr, &location, date, &params);
        // assert_datetime!(
        //     output.sunrise,
        //     expected_output.sunrise,
        //     &location,
        //     date,
        //     &params
        // );
        assert_datetime!(
            output.dhuhr,
            expected_output.dhuhr,
            &location,
            date,
            &params
        );
        dbg!("done");
        // assert_datetime!(output.asr, expected_output.asr, &location, date, &params);
        // assert_datetime!(
        //     output.sunset,
        //     expected_output.sunset,
        //     &location,
        //     date,
        //     &params
        // );
        // assert_datetime!(
        //     output.maghrib,
        //     expected_output.maghrib,
        //     &location,
        //     date,
        //     &params
        // );
        // assert_datetime!(output.isha, expected_output.isha, &location, date, &params);
        // assert_datetime!(
        //     output.midnight,
        //     expected_output.midnight,
        //     &location,
        //     date,
        //     &params
        // );
    }
}
