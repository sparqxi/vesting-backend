use mongodb::{bson::doc, error::Error, options::FindOneAndUpdateOptions, Client, Collection};

use crate::model::beneficiaries_model::Beneficiaries;
#[derive(Clone)]
pub struct Database {
    pub beneficiaries:Collection<Beneficiaries>
}

impl Database {
    pub async fn _init() -> Self {
        let url = dotenv::var("MONGO_URL").unwrap();
      let client =   Client::with_uri_str(url.clone()).await.unwrap();
       let db =client.database("test");
       let beneficiaries = db.collection("beneficiaries");
       Database {
        beneficiaries
    }} 

    pub async fn update_beneficiary(&self,address:String,claim_time:i64,claimed_token:f64) -> Result<Option<Beneficiaries>,Error>{

        let filter = doc! {"wallet": &address};
        let update = doc! {
            "$set": {"lastClaimTime": claim_time},
            "$inc": {"claimedTokens": claimed_token}
        };

        let old_beneficiary = match self.beneficiaries.find_one(filter.clone(), None).await {
            Ok(old) => old,
            Err(e) => {return Err(Error::custom(e.to_string()));},
        };
        if let Some(old) = old_beneficiary {
            if old.last_claim_time == claim_time {
                return Err(Error::custom("LastClaim time can't same with before"))
            }
        }
        let options = FindOneAndUpdateOptions::builder().return_document(mongodb::options::ReturnDocument::After).build();
        let result = self.beneficiaries.find_one_and_update(filter, update, options).await;
        match result {
            Ok(result) => {return Ok(result);},
            Err(e) =>{return Err(Error::custom(e.to_string()));}
        }
       
    }

    pub async fn get_beneficiary(&self,address:String) -> Result<Option<Beneficiaries>,Error> {
        let result = self.beneficiaries.find_one(doc! {"wallet":address}, None).await.ok().expect("Find failed");
        Ok(result)
    }
}