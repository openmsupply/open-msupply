use chrono::NaiveDate;
use repository::{DateFilter, EqualFilter, Gender, RepositoryError, SimpleStringFilter};

use crate::service_provider::{ServiceContext, ServiceProvider};

use super::{Patient, PatientFilter};

pub struct PatientSearch {
    pub code: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<Gender>,
}

pub struct PatientSearchResult {
    pub patient: Patient,
    /// Indicates how good the match was
    pub score: f64,
}

pub fn patient_search(
    ctx: &ServiceContext,
    service_provider: &ServiceProvider,
    store_id: String,
    input: PatientSearch,
) -> Result<Vec<PatientSearchResult>, RepositoryError> {
    let mut filter = PatientFilter::new();
    let PatientSearch {
        code,
        first_name,
        last_name,
        date_of_birth,
        gender,
    } = input;
    if let Some(code) = code {
        filter = filter.code(SimpleStringFilter::equal_to(&code));
    }
    if let Some(first_name) = first_name {
        filter = filter.first_name(SimpleStringFilter::equal_to(&first_name));
    }
    if let Some(last_name) = last_name {
        filter = filter.last_name(SimpleStringFilter::equal_to(&last_name));
    }
    if let Some(date_of_birth) = date_of_birth {
        filter = filter.date_of_birth(DateFilter::equal_to(date_of_birth));
    }
    if let Some(gender) = gender {
        filter = filter.gender(EqualFilter {
            equal_to: Some(gender),
            not_equal_to: None,
            equal_any: None,
            not_equal_all: None,
        });
    }

    let results: Vec<PatientSearchResult> = service_provider
        .patient_service
        .get_patients(ctx, &store_id, None, Some(filter), None)?
        .rows
        .into_iter()
        .map(|patient| PatientSearchResult {
            patient,
            score: 1.0,
        })
        .collect();
    Ok(results)
}
