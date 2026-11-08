//! SigmaOS-activitypub — comprehensive ActivityPub federation component.
//!
//! Provides ActivityPub protocol implementation for federated social networking,
//! including actors, activities, objects, and federation with other servers.

use std::collections::HashMap;

/// ActivityPub object type
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    /// Person actor
    Person,
    /// Organization actor
    Organization,
    /// Application actor
    Application,
    /// Note object
    Note,
    /// Article object
    Article,
    /// Image object
    Image,
    /// Video object
    Video,
    /// Audio object
    Audio,
    /// Document object
    Document,
    /// Event object
    Event,
    /// Place object
    Place,
    /// Ordered collection
    OrderedCollection,
    /// Collection
    Collection,
}

/// Activity type
#[derive(Debug, Clone, PartialEq)]
pub enum ActivityType {
    /// Create activity
    Create,
    /// Update activity
    Update,
    /// Delete activity
    Delete,
    /// Follow activity
    Follow,
    /// Undo activity
    Undo,
    /// Like activity
    Like,
    /// Announce activity
    Announce,
    /// Accept activity
    Accept,
    /// Reject activity
    Reject,
    /// Add activity
    Add,
    /// Remove activity
    Remove,
    /// Block activity
    Block,
    /// Flag activity
    Flag,
    /// View activity
    View,
}

/// ActivityPub actor
#[derive(Debug, Clone)]
pub struct Actor {
    /// Actor ID (URL)
    pub id: String,
    /// Actor type
    pub actor_type: ObjectType,
    /// Display name
    pub name: String,
    /// Preferred username
    pub preferred_username: String,
    /// Summary/description
    pub summary: String,
    /// URL
    pub url: String,
    /// Manually approved followers
    pub manually_approves_followers: bool,
    /// Published timestamp
    pub published: u64,
    /// Public key
    pub public_key: PublicKey,
    /// Endpoints
    pub endpoints: HashMap<String, String>,
    /// Inbox URL
    pub inbox: String,
    /// Outbox URL
    pub outbox: String,
    /// Followers URL
    pub followers: String,
    /// Following URL
    pub following: String,
    /// Featured URL
    pub featured: Option<String>,
    /// Icon
    pub icon: Option<String>,
    /// Image
    pub image: Option<String>,
    /// Attachment
    pub attachment: Vec<Attachment>,
}

/// Public key
#[derive(Debug, Clone)]
pub struct PublicKey {
    /// Key ID
    pub id: String,
    /// Key type
    pub key_type: String,
    /// Key owner
    pub owner: String,
    /// Public key material
    pub public_key_pem: String,
}

/// Attachment
#[derive(Debug, Clone)]
pub struct Attachment {
    /// Attachment type
    pub attachment_type: String,
    /// Media type
    pub media_type: String,
    /// URL
    pub url: String,
    /// Name
    pub name: String,
    /// Value
    pub value: Option<String>,
}

/// ActivityPub object
#[derive(Debug, Clone)]
pub struct Object {
    /// Object ID
    pub id: String,
    /// Object type
    pub object_type: ObjectType,
    /// Attributed to (actor)
    pub attributed_to: String,
    /// Published timestamp
    pub published: u64,
    /// Updated timestamp
    pub updated: Option<u64>,
    /// URL
    pub url: String,
    /// Name/title
    pub name: String,
    /// Content
    pub content: String,
    /// Summary
    pub summary: Option<String>,
    /// Media type
    pub media_type: String,
    /// Attachment
    pub attachment: Vec<Attachment>,
    /// Tag
    pub tag: Vec<Tag>,
    /// Audience
    pub audience: Option<String>,
    /// InReplyTo
    pub in_reply_to: Option<String>,
}

/// Tag
#[derive(Debug, Clone)]
pub struct Tag {
    /// Tag type
    pub tag_type: String,
    /// Tag name
    pub name: String,
    /// Tag href
    pub href: String,
}

/// ActivityPub activity
#[derive(Debug, Clone)]
pub struct Activity {
    /// Activity ID
    pub id: String,
    /// Activity type
    pub activity_type: ActivityType,
    /// Actor
    pub actor: String,
    /// Object
    pub object: Object,
    /// Target
    pub target: Option<String>,
    /// Result
    pub result: Option<String>,
    /// Published timestamp
    pub published: u64,
    /// To
    pub to: Vec<String>,
    /// CC
    pub cc: Vec<String>,
    /// BCC
    pub bcc: Vec<String>,
    /// Audience
    pub audience: Option<String>,
}

/// Federation server
#[derive(Debug, Clone)]
pub struct FederationServer {
    /// Server domain
    pub domain: String,
    /// Server actor
    pub actor: Actor,
    /// Known actors
    pub known_actors: HashMap<String, Actor>,
    /// Inbox queue
    pub inbox_queue: Vec<Activity>,
    /// Outbox queue
    pub outbox_queue: Vec<Activity>,
    /// Statistics
    pub stats: ServerStatistics,
}

/// Server statistics
#[derive(Debug, Clone, Default)]
pub struct ServerStatistics {
    /// Total activities received
    pub activities_received: u64,
    /// Total activities sent
    pub activities_sent: u64,
    /// Total actors known
    pub actors_known: u64,
    /// Total objects stored
    pub objects_stored: u64,
    /// Last activity timestamp
    pub last_activity: u64,
}

/// ActivityPub federation manager
#[derive(Debug, Clone)]
pub struct ActivityPubManager {
    /// Local actor
    pub local_actor: Actor,
    /// Known actors
    pub actors: HashMap<String, Actor>,
    /// Objects
    pub objects: HashMap<String, Object>,
    /// Activities
    pub activities: HashMap<String, Activity>,
    /// Follow relationships
    pub follows: HashMap<String, Vec<String>>,
    /// Like relationships
    pub likes: HashMap<String, Vec<String>>,
    /// Statistics
    pub stats: FederationStatistics,
}

/// Federation statistics
#[derive(Debug, Clone, Default)]
pub struct FederationStatistics {
    /// Total actors
    pub total_actors: u64,
    /// Total objects
    pub total_objects: u64,
    /// Total activities
    pub total_activities: u64,
    /// Total followers
    pub total_followers: u64,
    /// Total following
    pub total_following: u64,
    /// Total likes
    pub total_likes: u64,
}

impl Default for ActivityPubManager {
    fn default() -> Self {
        Self {
            local_actor: Actor {
                id: "https://local.instance/users/local".into(),
                actor_type: ObjectType::Person,
                name: "Local User".into(),
                preferred_username: "local".into(),
                summary: "Local user".into(),
                url: "https://local.instance/users/local".into(),
                manually_approves_followers: false,
                published: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                public_key: PublicKey {
                    id: "https://local.instance/users/local#main-key".into(),
                    key_type: "RsaSignature2017".into(),
                    owner: "https://local.instance/users/local".into(),
                    public_key_pem: "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----".into(),
                },
                endpoints: HashMap::new(),
                inbox: "https://local.instance/users/local/inbox".into(),
                outbox: "https://local.instance/users/local/outbox".into(),
                followers: "https://local.instance/users/local/followers".into(),
                following: "https://local.instance/users/local/following".into(),
                featured: None,
                icon: None,
                image: None,
                attachment: vec![],
            },
            actors: HashMap::new(),
            objects: HashMap::new(),
            activities: HashMap::new(),
            follows: HashMap::new(),
            likes: HashMap::new(),
            stats: FederationStatistics::default(),
        }
    }
}

impl ActivityPubManager {
    /// Create new ActivityPub manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with local actor
    pub fn with_actor(actor: Actor) -> Self {
        Self {
            local_actor: actor,
            ..Default::default()
        }
    }
    
    /// Add actor
    pub fn add_actor(&mut self, actor: Actor) -> String {
        let id = actor.id.clone();
        self.actors.insert(id.clone(), actor);
        self.stats.total_actors += 1;
        id
    }
    
    /// Remove actor
    pub fn remove_actor(&mut self, actor_id: &str) -> bool {
        if self.actors.remove(actor_id).is_some() {
            self.stats.total_actors = self.stats.total_actors.saturating_sub(1);
            return true;
        }
        false
    }
    
    /// Get actor
    pub fn get_actor(&self, actor_id: &str) -> Option<&Actor> {
        if actor_id == self.local_actor.id {
            Some(&self.local_actor)
        } else {
            self.actors.get(actor_id)
        }
    }
    
    /// Get all actors
    pub fn get_all_actors(&self) -> Vec<&Actor> {
        let mut actors = vec![&self.local_actor];
        actors.extend(self.actors.values());
        actors
    }
    
    /// Add object
    pub fn add_object(&mut self, object: Object) -> String {
        let id = object.id.clone();
        self.objects.insert(id.clone(), object);
        self.stats.total_objects += 1;
        id
    }
    
    /// Remove object
    pub fn remove_object(&mut self, object_id: &str) -> bool {
        if self.objects.remove(object_id).is_some() {
            self.stats.total_objects = self.stats.total_objects.saturating_sub(1);
            return true;
        }
        false
    }
    
    /// Get object
    pub fn get_object(&self, object_id: &str) -> Option<&Object> {
        self.objects.get(object_id)
    }
    
    /// Get all objects
    pub fn get_all_objects(&self) -> Vec<&Object> {
        self.objects.values().collect()
    }
    
    /// Get objects by type
    pub fn get_objects_by_type(&self, object_type: &ObjectType) -> Vec<&Object> {
        self.objects.values()
            .filter(|o| o.object_type == *object_type)
            .collect()
    }
    
    /// Get objects by actor
    pub fn get_objects_by_actor(&self, actor_id: &str) -> Vec<&Object> {
        self.objects.values()
            .filter(|o| o.attributed_to == actor_id)
            .collect()
    }
    
    /// Add activity
    pub fn add_activity(&mut self, activity: Activity) -> String {
        let id = activity.id.clone();
        self.activities.insert(id.clone(), activity);
        self.stats.total_activities += 1;
        id
    }
    
    /// Remove activity
    pub fn remove_activity(&mut self, activity_id: &str) -> bool {
        if self.activities.remove(activity_id).is_some() {
            self.stats.total_activities = self.stats.total_activities.saturating_sub(1);
            return true;
        }
        false
    }
    
    /// Get activity
    pub fn get_activity(&self, activity_id: &str) -> Option<&Activity> {
        self.activities.get(activity_id)
    }
    
    /// Get all activities
    pub fn get_all_activities(&self) -> Vec<&Activity> {
        self.activities.values().collect()
    }
    
    /// Get activities by type
    pub fn get_activities_by_type(&self, activity_type: &ActivityType) -> Vec<&Activity> {
        self.activities.values()
            .filter(|a| a.activity_type == *activity_type)
            .collect()
    }
    
    /// Get activities by actor
    pub fn get_activities_by_actor(&self, actor_id: &str) -> Vec<&Activity> {
        self.activities.values()
            .filter(|a| a.actor == actor_id)
            .collect()
    }
    
    /// Follow actor
    pub fn follow(&mut self, follower: &str, following: &str) -> bool {
        self.follows.entry(follower.to_string())
            .or_insert_with(Vec::new)
            .push(following.to_string());
        self.stats.total_following += 1;
        self.stats.total_followers += 1;
        true
    }
    
    /// Unfollow actor
    pub fn unfollow(&mut self, follower: &str, following: &str) -> bool {
        if let Some(following_list) = self.follows.get_mut(follower) {
            if let Some(pos) = following_list.iter().position(|f| f == following) {
                following_list.remove(pos);
                self.stats.total_following = self.stats.total_following.saturating_sub(1);
                self.stats.total_followers = self.stats.total_followers.saturating_sub(1);
                return true;
            }
        }
        false
    }
    
    /// Get followers of actor
    pub fn get_followers(&self, actor_id: &str) -> Vec<String> {
        self.follows.iter()
            .filter(|(_, following)| following.contains(&actor_id.to_string()))
            .map(|(follower, _)| follower.clone())
            .collect()
    }
    
    /// Get following of actor
    pub fn get_following(&self, actor_id: &str) -> Vec<String> {
        self.follows.get(actor_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Like object
    pub fn like(&mut self, actor_id: &str, object_id: &str) -> bool {
        self.likes.entry(object_id.to_string())
            .or_insert_with(Vec::new)
            .push(actor_id.to_string());
        self.stats.total_likes += 1;
        true
    }
    
    /// Unlike object
    pub fn unlike(&mut self, actor_id: &str, object_id: &str) -> bool {
        if let Some(likes) = self.likes.get_mut(object_id) {
            if let Some(pos) = likes.iter().position(|a| a == actor_id) {
                likes.remove(pos);
                self.stats.total_likes = self.stats.total_likes.saturating_sub(1);
                return true;
            }
        }
        false
    }
    
    /// Get likes for object
    pub fn get_likes(&self, object_id: &str) -> Vec<String> {
        self.likes.get(object_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Create note object
    pub fn create_note(&mut self, content: String, attributed_to: String) -> String {
        let object = Object {
            id: format!("https://local.instance/notes/{}", self.stats.total_objects + 1),
            object_type: ObjectType::Note,
            attributed_to,
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated: None,
            url: format!("https://local.instance/notes/{}", self.stats.total_objects + 1),
            name: String::new(),
            content,
            summary: None,
            media_type: "text/html".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        };
        
        self.add_object(object)
    }
    
    /// Create article object
    pub fn create_article(&mut self, title: String, content: String, attributed_to: String) -> String {
        let object = Object {
            id: format!("https://local.instance/articles/{}", self.stats.total_objects + 1),
            object_type: ObjectType::Article,
            attributed_to,
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated: None,
            url: format!("https://local.instance/articles/{}", self.stats.total_objects + 1),
            name: title,
            content,
            summary: None,
            media_type: "text/html".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        };
        
        self.add_object(object)
    }
    
    /// Create image object
    pub fn create_image(&mut self, url: String, attributed_to: String) -> String {
        let object = Object {
            id: format!("https://local.instance/images/{}", self.stats.total_objects + 1),
            object_type: ObjectType::Image,
            attributed_to,
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated: None,
            url: url.clone(),
            name: String::new(),
            content: String::new(),
            summary: None,
            media_type: "image/jpeg".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        };
        
        self.add_object(object)
    }
    
    /// Create create activity
    pub fn create_activity(&mut self, actor: String, object_id: String) -> String {
        let object = self.get_object(&object_id).cloned().unwrap_or(Object {
            id: object_id.clone(),
            object_type: ObjectType::Note,
            attributed_to: actor.clone(),
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated: None,
            url: object_id.clone(),
            name: String::new(),
            content: String::new(),
            summary: None,
            media_type: "text/plain".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        });
        
        let activity = Activity {
            id: format!("https://local.instance/activities/{}", self.stats.total_activities + 1),
            activity_type: ActivityType::Create,
            actor,
            object,
            target: None,
            result: None,
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            to: vec![],
            cc: vec![],
            bcc: vec![],
            audience: None,
        };
        
        self.add_activity(activity)
    }
    
    /// Create follow activity
    pub fn create_follow_activity(&mut self, actor: String, target: String) -> String {
        let object = Object {
            id: format!("https://local.instance/follows/{}", self.stats.total_activities + 1),
            object_type: ObjectType::Person,
            attributed_to: actor.clone(),
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated: None,
            url: target.clone(),
            name: String::new(),
            content: String::new(),
            summary: None,
            media_type: "text/plain".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        };
        
        let activity = Activity {
            id: format!("https://local.instance/activities/{}", self.stats.total_activities + 1),
            activity_type: ActivityType::Follow,
            actor,
            object,
            target: Some(target),
            result: None,
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            to: vec![],
            cc: vec![],
            bcc: vec![],
            audience: None,
        };
        
        self.add_activity(activity)
    }
    
    /// Create like activity
    pub fn create_like_activity(&mut self, actor: String, object_id: String) -> String {
        let object = self.get_object(&object_id).cloned().unwrap_or(Object {
            id: object_id.clone(),
            object_type: ObjectType::Note,
            attributed_to: actor.clone(),
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated: None,
            url: object_id.clone(),
            name: String::new(),
            content: String::new(),
            summary: None,
            media_type: "text/plain".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        });
        
        let activity = Activity {
            id: format!("https://local.instance/activities/{}", self.stats.total_activities + 1),
            activity_type: ActivityType::Like,
            actor,
            object,
            target: None,
            result: None,
            published: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            to: vec![],
            cc: vec![],
            bcc: vec![],
            audience: None,
        };
        
        self.add_activity(activity)
    }
    
    /// Get federation statistics
    pub fn get_statistics(&self) -> &FederationStatistics {
        &self.stats
    }
    
    /// Export federation data
    pub fn export_data(&self) -> FederationExport {
        FederationExport {
            local_actor: self.local_actor.clone(),
            actors: self.actors.values().cloned().collect(),
            objects: self.objects.values().cloned().collect(),
            activities: self.activities.values().cloned().collect(),
            follows: self.follows.clone(),
            likes: self.likes.clone(),
        }
    }
    
    /// Import federation data
    pub fn import_data(&mut self, export: FederationExport) {
        self.local_actor = export.local_actor;
        
        for actor in export.actors {
            self.actors.insert(actor.id.clone(), actor);
        }
        
        for object in export.objects {
            self.objects.insert(object.id.clone(), object);
        }
        
        for activity in export.activities {
            self.activities.insert(activity.id.clone(), activity);
        }
        
        self.follows = export.follows;
        self.likes = export.likes;
        
        // Recalculate statistics
        self.stats.total_actors = self.actors.len() as u64;
        self.stats.total_objects = self.objects.len() as u64;
        self.stats.total_activities = self.activities.len() as u64;
        self.stats.total_followers = self.follows.values().map(|v| v.len()).sum::<usize>() as u64;
        self.stats.total_following = self.follows.len() as u64;
        self.stats.total_likes = self.likes.values().map(|v| v.len()).sum::<usize>() as u64;
    }
}

/// Federation export data
#[derive(Debug, Clone)]
pub struct FederationExport {
    /// Local actor
    pub local_actor: Actor,
    /// Actors
    pub actors: Vec<Actor>,
    /// Objects
    pub objects: Vec<Object>,
    /// Activities
    pub activities: Vec<Activity>,
    /// Follows
    pub follows: HashMap<String, Vec<String>>,
    /// Likes
    pub likes: HashMap<String, Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_object_type() {
        let types = vec![
            ObjectType::Person,
            ObjectType::Organization,
            ObjectType::Application,
            ObjectType::Note,
            ObjectType::Article,
            ObjectType::Image,
            ObjectType::Video,
            ObjectType::Audio,
            ObjectType::Document,
            ObjectType::Event,
            ObjectType::Place,
            ObjectType::OrderedCollection,
            ObjectType::Collection,
        ];
        assert_eq!(types.len(), 13);
    }
    
    #[test]
    fn test_activity_type() {
        let types = vec![
            ActivityType::Create,
            ActivityType::Update,
            ActivityType::Delete,
            ActivityType::Follow,
            ActivityType::Undo,
            ActivityType::Like,
            ActivityType::Announce,
            ActivityType::Accept,
            ActivityType::Reject,
            ActivityType::Add,
            ActivityType::Remove,
            ActivityType::Block,
            ActivityType::Flag,
            ActivityType::View,
        ];
        assert_eq!(types.len(), 14);
    }
    
    #[test]
    fn test_actor_creation() {
        let actor = Actor {
            id: "https://example.com/users/test".into(),
            actor_type: ObjectType::Person,
            name: "Test User".into(),
            preferred_username: "test".into(),
            summary: "Test user".into(),
            url: "https://example.com/users/test".into(),
            manually_approves_followers: false,
            published: 1234567890,
            public_key: PublicKey {
                id: "https://example.com/users/test#main-key".into(),
                key_type: "RsaSignature2017".into(),
                owner: "https://example.com/users/test".into(),
                public_key_pem: "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----".into(),
            },
            endpoints: HashMap::new(),
            inbox: "https://example.com/users/test/inbox".into(),
            outbox: "https://example.com/users/test/outbox".into(),
            followers: "https://example.com/users/test/followers".into(),
            following: "https://example.com/users/test/following".into(),
            featured: None,
            icon: None,
            image: None,
            attachment: vec![],
        };
        
        assert_eq!(actor.id, "https://example.com/users/test");
        assert_eq!(actor.actor_type, ObjectType::Person);
    }
    
    #[test]
    fn test_object_creation() {
        let object = Object {
            id: "https://example.com/notes/1".into(),
            object_type: ObjectType::Note,
            attributed_to: "https://example.com/users/test".into(),
            published: 1234567890,
            updated: None,
            url: "https://example.com/notes/1".into(),
            name: String::new(),
            content: "Test note".into(),
            summary: None,
            media_type: "text/html".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        };
        
        assert_eq!(object.id, "https://example.com/notes/1");
        assert_eq!(object.object_type, ObjectType::Note);
        assert_eq!(object.content, "Test note");
    }
    
    #[test]
    fn test_activity_creation() {
        let activity = Activity {
            id: "https://example.com/activities/1".into(),
            activity_type: ActivityType::Create,
            actor: "https://example.com/users/test".into(),
            object: Object {
                id: "https://example.com/notes/1".into(),
                object_type: ObjectType::Note,
                attributed_to: "https://example.com/users/test".into(),
                published: 1234567890,
                updated: None,
                url: "https://example.com/notes/1".into(),
                name: String::new(),
                content: "Test note".into(),
                summary: None,
                media_type: "text/html".into(),
                attachment: vec![],
                tag: vec![],
                audience: None,
                in_reply_to: None,
            },
            target: None,
            result: None,
            published: 1234567890,
            to: vec![],
            cc: vec![],
            bcc: vec![],
            audience: None,
        };
        
        assert_eq!(activity.id, "https://example.com/activities/1");
        assert_eq!(activity.activity_type, ActivityType::Create);
    }
    
    #[test]
    fn test_manager_creation() {
        let manager = ActivityPubManager::new();
        assert_eq!(manager.get_statistics().total_actors, 0);
        assert_eq!(manager.get_statistics().total_objects, 0);
    }
    
    #[test]
    fn test_add_actor() {
        let mut manager = ActivityPubManager::new();
        
        let actor = Actor {
            id: "https://example.com/users/test".into(),
            actor_type: ObjectType::Person,
            name: "Test User".into(),
            preferred_username: "test".into(),
            summary: "Test user".into(),
            url: "https://example.com/users/test".into(),
            manually_approves_followers: false,
            published: 1234567890,
            public_key: PublicKey {
                id: "https://example.com/users/test#main-key".into(),
                key_type: "RsaSignature2017".into(),
                owner: "https://example.com/users/test".into(),
                public_key_pem: "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----".into(),
            },
            endpoints: HashMap::new(),
            inbox: "https://example.com/users/test/inbox".into(),
            outbox: "https://example.com/users/test/outbox".into(),
            followers: "https://example.com/users/test/followers".into(),
            following: "https://example.com/users/test/following".into(),
            featured: None,
            icon: None,
            image: None,
            attachment: vec![],
        };
        
        manager.add_actor(actor);
        assert_eq!(manager.get_statistics().total_actors, 1);
        assert!(manager.get_actor("https://example.com/users/test").is_some());
    }
    
    #[test]
    fn test_add_object() {
        let mut manager = ActivityPubManager::new();
        
        let object = Object {
            id: "https://example.com/notes/1".into(),
            object_type: ObjectType::Note,
            attributed_to: "https://example.com/users/test".into(),
            published: 1234567890,
            updated: None,
            url: "https://example.com/notes/1".into(),
            name: String::new(),
            content: "Test note".into(),
            summary: None,
            media_type: "text/html".into(),
            attachment: vec![],
            tag: vec![],
            audience: None,
            in_reply_to: None,
        };
        
        manager.add_object(object);
        assert_eq!(manager.get_statistics().total_objects, 1);
        assert!(manager.get_object("https://example.com/notes/1").is_some());
    }
    
    #[test]
    fn test_follow_unfollow() {
        let mut manager = ActivityPubManager::new();
        
        manager.follow("https://example.com/users/user1", "https://example.com/users/user2");
        assert!(manager.get_followers("https://example.com/users/user2").contains(&"https://example.com/users/user1".to_string()));
        
        manager.unfollow("https://example.com/users/user1", "https://example.com/users/user2");
        assert!(!manager.get_followers("https://example.com/users/user2").contains(&"https://example.com/users/user1".to_string()));
    }
    
    #[test]
    fn test_like_unlike() {
        let mut manager = ActivityPubManager::new();
        
        manager.like("https://example.com/users/user1", "https://example.com/notes/1");
        assert!(manager.get_likes("https://example.com/notes/1").contains(&"https://example.com/users/user1".to_string()));
        
        manager.unlike("https://example.com/users/user1", "https://example.com/notes/1");
        assert!(!manager.get_likes("https://example.com/notes/1").contains(&"https://example.com/users/user1".to_string()));
    }
    
    #[test]
    fn test_create_note() {
        let mut manager = ActivityPubManager::new();
        
        let note_id = manager.create_note("Test note content".into(), "https://example.com/users/test".into());
        assert!(manager.get_object(&note_id).is_some());
        assert_eq!(manager.get_object(&note_id).unwrap().content, "Test note content");
    }
    
    #[test]
    fn test_create_activity() {
        let mut manager = ActivityPubManager::new();
        
        let note_id = manager.create_note("Test note".into(), "https://example.com/users/test".into());
        let activity_id = manager.create_activity("https://example.com/users/test".into(), note_id);
        
        assert!(manager.get_activity(&activity_id).is_some());
        assert_eq!(manager.get_activity(&activity_id).unwrap().activity_type, ActivityType::Create);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = ActivityPubManager::new();
        manager.create_note("Test note".into(), "https://example.com/users/test".into());
        manager.follow("https://example.com/users/user1", "https://example.com/users/user2");
        
        let export = manager.export_data();
        
        let mut manager2 = ActivityPubManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.get_statistics().total_objects, manager.get_statistics().total_objects);
        assert_eq!(manager2.get_statistics().total_followers, manager.get_statistics().total_followers);
    }
}
