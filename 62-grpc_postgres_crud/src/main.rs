
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};
use sqlx::{Pool, Postgres, Row};
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

pub mod user {
    tonic::include_proto!("user");
}

use user::user_service_server::{UserService, UserServiceServer};
use user::*;

#[derive(Clone)]
struct UserServiceImpl { db: Pool<Postgres> }

impl UserServiceImpl {
    fn new(db: Pool<Postgres>) -> Self { Self { db } }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(
        &self, request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        info!("create_user name={} email={}", req.name, req.email);

        let row = sqlx::query("INSERT INTO users (name,email) VALUES ($1,$2) RETURNING id,name,email")
            .bind(&req.name).bind(&req.email)
            .fetch_one(&self.db).await
            .map_err(|e| { error!(?e); Status::internal("insert failed") })?;

        let user = User { id: row.get("id"), name: row.get("name"), email: row.get("email") };
        Ok(Response::new(UserResponse { user: Some(user) }))
    }

    async fn get_user(
        &self, request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        info!("get_user id={}", req.id);

        let row = sqlx::query("SELECT id,name,email FROM users WHERE id=$1")
            .bind(req.id).fetch_optional(&self.db).await
            .map_err(|e| { error!(?e); Status::internal("query failed") })?;

        let row = match row { Some(r) => r, None => return Err(Status::not_found("not found")) };

        let user = User { id: row.get("id"), name: row.get("name"), email: row.get("email") };
        Ok(Response::new(UserResponse { user: Some(user) }))
    }

    async fn list_users(
        &self, _req: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        info!("list_users");
        let rows = sqlx::query("SELECT id,name,email FROM users ORDER BY id")
            .fetch_all(&self.db).await
            .map_err(|e| { error!(?e); Status::internal("list failed") })?;

        let users = rows.into_iter().map(|r| User {
            id: r.get("id"), name: r.get("name"), email: r.get("email")
        }).collect();

        Ok(Response::new(ListUsersResponse { users }))
    }

    async fn update_user(
        &self, request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        info!("update_user id={} name={} email={}", req.id, req.name, req.email);

        let row = sqlx::query("UPDATE users SET name=$1,email=$2 WHERE id=$3 RETURNING id,name,email")
            .bind(&req.name).bind(&req.email).bind(req.id)
            .fetch_optional(&self.db).await
            .map_err(|e| { error!(?e); Status::internal("update failed") })?;

        let row = match row { Some(r) => r, None => return Err(Status::not_found("not found")) };

        let user = User { id: row.get("id"), name: row.get("name"), email: row.get("email") };
        Ok(Response::new(UserResponse { user: Some(user) }))
    }

    async fn delete_user(
        &self, request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let req = request.into_inner();
        info!("delete_user id={}", req.id);

        let result = sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(req.id).execute(&self.db).await
            .map_err(|e| { error!(?e); Status::internal("delete failed") })?;

        Ok(Response::new(DeleteUserResponse { success: result.rows_affected() > 0 }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); //.env

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .compact()
        .init();

    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?).await?;


    let addr: SocketAddr = "0.0.0.0:50055".parse()?;
    info!("🚀 gRPC server running on {}", addr);

    let svc = UserServiceImpl::new(db);

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr).await?;
    Ok(())
}
