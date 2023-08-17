use chrono::{Datelike, NaiveDate, NaiveDateTime};
use pray_times::{
    types::{Location, Parameters, PraytimesOutput},
    Calculator,
};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestCase {
    inputs: Inputs,
    #[serde(rename = "originalOutput")]
    expected_output: PraytimesOutput,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Inputs {
    params: Parameters,
    location: Location,
    #[serde(serialize_with = "serialize_date")]
    #[serde(deserialize_with = "deserialize_date")]
    date: NaiveDate,
}

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(3))?;
    seq.serialize_element(&date.year())?;
    seq.serialize_element(&date.month())?;
    seq.serialize_element(&date.day())?;
    seq.end()
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Vec::<u32>::deserialize(deserializer)?;
    NaiveDate::from_ymd_opt(vec[0].try_into().unwrap(), vec[1], vec[2])
        .ok_or(serde::de::Error::custom("Invalid date"))
}

fn assert_prayertime(
    name: &str,
    real: Option<NaiveDateTime>,
    expected: Option<NaiveDateTime>,
    location: &Location,
    date: chrono::NaiveDate,
    params: &Parameters,
    expected_output: &PraytimesOutput,
    output: &PraytimesOutput,
) {
    let res = match (&real, &expected) {
        (None, None) => true,
        (Some(real), Some(expected)) => {
            (real.timestamp_millis() - expected.timestamp_millis()).abs() < 5000
        }
        _ => false,
    };

    assert!(
        res,
        "\n\ninvalid {name}\t\t{real:?}\nexpected\t{expected:?} \n\n\nparameters : {params:#?}\nlocation: {location:#?}\ndate: {date:?}\nexpected output : {expected_output:#?}\n output : {output:#?}",
        real = real,
        expected = expected
    );
}

#[test]
fn should_match_with_the_main() {
    let data = std::fs::read_to_string("./assets/test-data.json").unwrap();
    // let binding = serde_json::from_str::<Value>(&data).unwrap();
    // let mut data = binding.as_array().unwrap();
    let data = serde_json::from_str::<Vec<TestCase>>(&data).unwrap();

    // let case = methods::SHIA_ITHNA_ASHARI_LEVA_INSTITUTE_QUM;
    // let json = serde_json::ser::to_string_pretty(&case).unwrap();
    println!("{:#?}", &data);
    for TestCase {
        inputs: Inputs {
            params,
            location,
            date,
        },
        expected_output,
    } in data
    {
        let output: PraytimesOutput = Calculator::new(params.clone()).calculate(&location, &date);
        assert_prayertime(
            "sunrise",
            output.sunrise,
            expected_output.sunrise,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );
        assert_prayertime(
            "dhuhr",
            output.dhuhr,
            expected_output.dhuhr,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );
        assert_prayertime(
            "sunset",
            output.sunset,
            expected_output.sunset,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );

        assert_prayertime(
            "fajr",
            output.fajr,
            expected_output.fajr,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );

        assert_prayertime(
            "imsak",
            output.imsak,
            expected_output.imsak,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );

        assert_prayertime(
            "maghrib",
            output.maghrib,
            expected_output.maghrib,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );
        assert_prayertime(
            "isha",
            output.isha,
            expected_output.isha,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );
        assert_prayertime(
            "midnight",
            output.midnight,
            expected_output.midnight,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );
        assert_prayertime(
            "asr",
            output.asr,
            expected_output.asr,
            &location,
            date,
            &params,
            &expected_output,
            &output,
        );
        dbg!("done");
    }
}
