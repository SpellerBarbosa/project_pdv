    use actix_web::{ post, web, HttpResponse, Responder };
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::api::data::db::init_db;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    user: String,
    password: String,
    role: String
}

#[post("/register_user")]
pub async fn register_user(user: web::Json<User>) ->impl Responder {
    let user = user.into_inner();

    if user.name.is_empty() ||
       user.user.is_empty() || 
       user.password.is_empty() ||
       user.role.is_empty() {
        return  HttpResponse::BadRequest().json(json!({
            "message":"Preencha todos os campos!"
        }));        
       }

       //conexão com banco de dados
     let conn = match init_db() {
         Ok(c) => c,
         Err(e) => {
            eprint!("Erro ao conectar no banco de dados: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message":"Erro ao conectar ao banco de dados."
            }));
         }
     };

     //verifica se o usuario já existe
    let mut stmt = match conn.prepare("SELECT COUNT(*) FROM users WHERE user = ?1") {
        Ok(s) => s,
        Err(e) =>{
            eprint!("Erro na query {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"message":"erro interno ao preparar consulta"}));
        }
    };

    let count:i64 = match stmt.query_row([&user.user], |row| row.get(0)) {
        Ok(s) => s,
        Err(_) => 0,
    };

    if count > 0 {
        return HttpResponse::BadRequest().json(json!({
            "message":"Usuário já existe"
        }));
    }
        // faz a insersão do usuario no banco de dados
    let result = conn.execute("INSERT INTO users (name, user, password, role) VALUES (?1, ?2, ?3, ?4)",
     (&user.name, &user.user, &user.password, &user.role),
    );
        // envia a resposta para o usuario
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message":"Usuário cadastrado com sucesso"
        })),
        Err(e)=>{
            eprintln!("Erro ao inserir usuario {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "message":"Erro ao cadastrar usuario"
            }))
        }
    }
}