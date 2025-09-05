use actix_web::{ get, HttpResponse, Responder };
use serde_json::json;
use crate::api::data::db::init_db;



#[get("/has-admin")]
pub async fn has_admin()-> impl Responder{
    
    //conexão com o banco de dados
    let conn = match init_db() {
        Ok(s) => s,
        Err(e) => {
            eprint!("Erro ao conectar ao banco de dados, {:?} ", e);
            return HttpResponse::InternalServerError().json(json!({
                "message":"falha ao conectar ao banco de dados"
            }));
        }
    };

    //busca no banco de dados
    let mut admin_exist = match conn.prepare("SELECT 1 FROM users WHERE role = 'admin'") {
        Ok(s) => s,
        Err(e) =>{
            eprintln!("Erro na query {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message":"erro interno ao preparar consulta"
            }));
        }
    };

    let count:i64 = match admin_exist.query_row([], |row|row.get(0)) {
       Ok(s)=> s,
       Err(_) => 0 
    };

    if count > 0 {
        return HttpResponse::Ok().json(json!({
            "has_admin": true
        }));
    }else {
        return  HttpResponse::Ok().json(json!({
            "has_admin": false
        }));
    }

}