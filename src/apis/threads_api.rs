/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_thread_mention_counts_by_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetThreadMentionCountsByChannelError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserThreadError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserThreadsError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_thread_unread_by_post_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetThreadUnreadByPostIdError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`start_following_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartFollowingThreadError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stop_following_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopFollowingThreadError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_thread_read_for_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateThreadReadForUserError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_threads_read_for_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateThreadsReadForUserError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}


/// Get all unread mention counts from followed threads  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 
pub async fn get_thread_mention_counts_by_channel(configuration: &configuration::Configuration, user_id: &str, team_id: &str) -> Result<(), Error<GetThreadMentionCountsByChannelError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads/mention_counts", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetThreadMentionCountsByChannelError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a thread  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 
pub async fn get_user_thread(configuration: &configuration::Configuration, user_id: &str, team_id: &str, thread_id: &str) -> Result<(), Error<GetUserThreadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads/{thread_id}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id), thread_id=crate::apis::urlencode(thread_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetUserThreadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all threads that user is following  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 
pub async fn get_user_threads(configuration: &configuration::Configuration, user_id: &str, team_id: &str, since: Option<i64>, deleted: Option<bool>, extended: Option<bool>, page: Option<i64>, page_size: Option<i64>, totals_only: Option<bool>, threads_only: Option<bool>) -> Result<crate::models::UserThreads, Error<GetUserThreadsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = deleted {
        local_var_req_builder = local_var_req_builder.query(&[("deleted", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = extended {
        local_var_req_builder = local_var_req_builder.query(&[("extended", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = totals_only {
        local_var_req_builder = local_var_req_builder.query(&[("totalsOnly", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = threads_only {
        local_var_req_builder = local_var_req_builder.query(&[("threadsOnly", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserThreadsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Mark a thread that user is following as unread  __Minimum server version__: 6.7  ##### Permissions Must have `read_channel` permission for the channel the thread is in or if the channel is public, have the `read_public_channels` permission for the team.  Must have `edit_other_users` permission if the user is not the one marking the thread for himself. 
pub async fn set_thread_unread_by_post_id(configuration: &configuration::Configuration, user_id: &str, team_id: &str, thread_id: &str, post_id: &str) -> Result<(), Error<SetThreadUnreadByPostIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads/{thread_id}/set_unread/{post_id}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id), thread_id=crate::apis::urlencode(thread_id), post_id=crate::apis::urlencode(post_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SetThreadUnreadByPostIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Start following a thread  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 
pub async fn start_following_thread(configuration: &configuration::Configuration, user_id: &str, team_id: &str, thread_id: &str) -> Result<(), Error<StartFollowingThreadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads/{thread_id}/following", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id), thread_id=crate::apis::urlencode(thread_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StartFollowingThreadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Stop following a thread  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 
pub async fn stop_following_thread(configuration: &configuration::Configuration, user_id: &str, team_id: &str, thread_id: &str) -> Result<(), Error<StopFollowingThreadError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads/{thread_id}/following", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id), thread_id=crate::apis::urlencode(thread_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StopFollowingThreadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Mark a thread that user is following as read  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 
pub async fn update_thread_read_for_user(configuration: &configuration::Configuration, user_id: &str, team_id: &str, thread_id: &str, timestamp: &str) -> Result<(), Error<UpdateThreadReadForUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads/{thread_id}/read/{timestamp}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id), thread_id=crate::apis::urlencode(thread_id), timestamp=crate::apis::urlencode(timestamp));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateThreadReadForUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Mark all threads that user is following as read  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 
pub async fn update_threads_read_for_user(configuration: &configuration::Configuration, user_id: &str, team_id: &str) -> Result<(), Error<UpdateThreadsReadForUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/{user_id}/teams/{team_id}/threads/read", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), team_id=crate::apis::urlencode(team_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateThreadsReadForUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

