struct Object(BTreeMap<String, serde_json::Value>);


///   "$schema": "http://json-schema.org/draft-04/schema#",
///   "title": "JSON API Schema",
///   "description": "This is a schema for responses in the JSON API format. For more, see http://jsonapi.org",
///   "oneOf": [
///     {
///       "$ref": "#/definitions/success"
///     },
///     {
///       "$ref": "#/definitions/failure"
///     },
///     {
///       "$ref": "#/definitions/info"
///     }
///   ],
enum Document {
    Success(Success),
    Failure(Failure),
    Info(Info),
}


///     "success": {
///       "type": "object",
///       "required": [
///         "data"
///       ],
///       "properties": {
///         "data": {
///           "$ref": "#/definitions/data"
///         },
///         "included": {
///           "description": "To reduce the number of HTTP requests, servers **MAY** allow responses that include related resources along with the requested primary resources. Such responses are called \"compound documents\".",
///           "type": "array",
///           "items": {
///             "$ref": "#/definitions/resource"
///           },
///           "uniqueItems": true
///         },
///         "meta": {
///           "$ref": "#/definitions/meta"
///         },
///         "links": {
///           "description": "Link members related to the primary data.",
///           "allOf": [
///             {
///               "$ref": "#/definitions/links"
///             },
///             {
///               "$ref": "#/definitions/pagination"
///             }
///           ]
///         },
///         "jsonapi": {
///           "$ref": "#/definitions/jsonapi"
///         }
///       },
///       "additionalProperties": false
///     },
struct Success {
    data: Data,
    included: Option<Vec<Included>>,
    meta: Option<Meta>,
    links: Option<LinksAndPagination>,
    jsonapi: Option<JsonAPI>,
}


///     "failure": {
///       "type": "object",
///       "required": [
///         "errors"
///       ],
///       "properties": {
///         "errors": {
///           "type": "array",
///           "items": {
///             "$ref": "#/definitions/error"
///           },
///           "uniqueItems": true
///         },
///         "meta": {
///           "$ref": "#/definitions/meta"
///         },
///         "jsonapi": {
///           "$ref": "#/definitions/jsonapi"
///         }
///       },
///       "additionalProperties": false
///     },
struct Failure {
    errors: Vec<Error>,
    meta: Option<Meta>,
    jsonapi: Option<JsonAPI>,
}


///     "info": {
///       "type": "object",
///       "required": [
///         "meta"
///       ],
///       "properties": {
///         "meta": {
///           "$ref": "#/definitions/meta"
///         },
///         "links": {
///           "$ref": "#/definitions/links"
///         },
///         "jsonapi": {
///           "$ref": "#/definitions/jsonapi"
///         }
///       },
///       "additionalProperties": false
///     },
struct Info {
    meta: Meta,
    links: Option<Links>,
    jsonapi: Option<JsonAPI>,
}


///     "meta": {
///       "description": "Non-standard meta-information that can not be represented as an attribute or relationship.",
///       "type": "object",
///       "additionalProperties": true
///     },
struct Meta(Object);


///     "data": {
///       "description": "The document's \"primary data\" is a representation of the resource or collection of resources targeted by a request.",
///       "oneOf": [
///         {
///           "$ref": "#/definitions/resource"
///         },
///         {
///           "description": "An array of resource objects, an array of resource identifier objects, or an empty array ([]), for requests that target resource collections.",
///           "type": "array",
///           "items": {
///             "$ref": "#/definitions/resource"
///           },
///           "uniqueItems": true
///         }
///       ]
///     },
enum Data {
    Single(Resource),
    Array(Vec<Resource>),
}


///     "resource": {
///       "description": "\"Resource objects\" appear in a JSON API document to represent resources.",
///       "type": "object",
///       "required": [
///         "type",
///         "id"
///       ],
///       "properties": {
///         "type": {
///           "type": "string"
///         },
///         "id": {
///           "type": "string"
///         },
///         "attributes": {
///           "$ref": "#/definitions/attributes"
///         },
///         "relationships": {
///           "$ref": "#/definitions/relationships"
///         },
///         "links": {
///           "$ref": "#/definitions/links"
///         },
///         "meta": {
///           "$ref": "#/definitions/meta"
///         }
///       },
///       "additionalProperties": false
///     },
struct Resource {
    type_: String,
    id: String,
    attributes: Option<Attributes>,
    relationships: Option<Relationships>,
    links: Option<Links>,
    meta: Option<Meta>,
}


///     "links": {
///       "description": "A resource object **MAY** contain references to other resource objects (\"relationships\"). Relationships may be to-one or to-many. Relationships can be specified by including a member in a resource's links object.",
///       "type": "object",
///       "properties": {
///         "self": {
///           "description": "A `self` member, whose value is a URL for the relationship itself (a \"relationship URL\"). This URL allows the client to directly manipulate the relationship. For example, it would allow a client to remove an `author` from an `article` without deleting the people resource itself.",
///           "type": "string",
///           "format": "uri"
///         },
///         "related": {
///           "$ref": "#/definitions/link"
///         }
///       },
///       "additionalProperties": true
///     },
struct Links {
    self_: Option<Uri>,
    related: Option<Link>,
    links: Option<Object>,
}


///     "link": {
///       "description": "A link **MUST** be represented as either: a string containing the link's URL or a link object.",
///       "oneOf": [
///         {
///           "description": "A string containing the link's URL.",
///           "type": "string",
///           "format": "uri"
///         },
///         {
///           "type": "object",
///           "required": [
///             "href"
///           ],
///           "properties": {
///             "href": {
///               "description": "A string containing the link's URL.",
///               "type": "string",
///               "format": "uri"
///             },
///             "meta": {
///               "$ref": "#/definitions/meta"
///             }
///           }
///         }
///       ]
///     },
enum Link {
    Url(Uri),
    Object {
        href: Uri,
        meta: Option<Meta>,
    },
}


///     "attributes": {
///       "description": "Members of the attributes object (\"attributes\") represent information about the resource object in which it's defined.",
///       "type": "object",
///       "patternProperties": {
///         "^(?!relationships$|links$)\\w[-\\w_]*$": {
///           "description": "Attributes may contain any valid JSON value."
///         }
///       },
///       "additionalProperties": false
///     },
struct Attributes {
    attributes: Object,
}

impl Attributes {
    fn new(attributes: Object) -> Attributes {
        for (key, _) in attributes.iter() {
            if not regex.search("^(?!relationships$|links$)\\w[-\\w_]*$", &key) {
                panic!("invalid attribute name");
            }
        }

        Attributes {
            attributes: attributes,
        }
    }
}


///     "relationships": {
///       "description": "Members of the relationships object (\"relationships\") represent references from the resource object in which it's defined to other resource objects.",
///       "type": "object",
///       "patternProperties": {
///         "^\\w[-\\w_]*$": {
///           "properties": {
///             "links": {
///               "$ref": "#/definitions/links"
///             },
///             "data": {
///               "description": "Member, whose value represents \"resource linkage\".",
///               "oneOf": [
///                 {
///                   "$ref": "#/definitions/relationshipToOne"
///                 },
///                 {
///                   "$ref": "#/definitions/relationshipToMany"
///                 }
///               ]
///             },
///             "meta": {
///               "$ref": "#/definitions/meta"
///             }
///           },
///           "additionalProperties": false
///         }
///       },
///       "additionalProperties": false
///     },
///     "relationshipToOne": {
///       "description": "References to other resource objects in a to-one (\"relationship\"). Relationships can be specified by including a member in a resource's links object.",
///       "anyOf": [
///         {
///           "$ref": "#/definitions/empty"
///         },
///         {
///           "$ref": "#/definitions/linkage"
///         }
///       ]
///     },
///     "relationshipToMany": {
///       "description": "An array of objects each containing \"type\" and \"id\" members for to-many relationships.",
///       "type": "array",
///       "items": {
///         "$ref": "#/definitions/linkage"
///       },
///       "uniqueItems": true
///     },
///     "empty": {
///       "description": "Describes an empty to-one relationship.",
///       "type": ["object", "null"],
///       "properties": {},
///       "additionalProperties": false
///     },
struct Relationships {
    relationships: BTreeMap<String, Relationship>,
}

impl Relationships {
    fn new(relationships: BTreeMap<String, Relationships>) -> Relationships {
        for (key, _) in relationships.iter() {
            if not regex.search("^\\w[-\\w_]*$", &key) {
                panic!("invalid relationship name");
            }
        }

        Relationships {
            relationships: relationships,
        }
    }
}

struct Relationship {
    links: Option<Links>,
    data: RelationshipData,
    meta: Option<Meta>,
}

enum RelationshipData {
    Empty,
    Single(Linkage),
    Array(Vec<Linkage>),
}


///     "linkage": {
///       "description": "The \"type\" and \"id\" to non-empty members.",
///       "type": "object",
///       "required": [
///         "type",
///         "id"
///       ],
///       "properties": {
///         "type": {
///           "type": "string"
///         },
///         "id": {
///           "type": "string"
///         }
///       },
///       "additionalProperties": false
///     },
struct Linkage {
    type_: String,
    id: String,
}


///     "pagination": {
///       "type": "object",
///       "properties": {
///         "first": {
///           "description": "The first page of data",
///           "oneOf": [
///             { "type": "string", "format": "uri" },
///             { "type": "null" }
///           ]
///         },
///         "last": {
///           "description": "The last page of data",
///           "oneOf": [
///             { "type": "string", "format": "uri" },
///             { "type": "null" }
///           ]
///         },
///         "prev": {
///           "description": "The previous page of data",
///           "oneOf": [
///             { "type": "string", "format": "uri" },
///             { "type": "null" }
///           ]
///         },
///         "next": {
///           "description": "The next page of data",
///           "oneOf": [
///             { "type": "string", "format": "uri" },
///             { "type": "null" }
///           ]
///         }
///       }
///     },
struct Pagination {
    first: Option<Uri>,
    last: Option<Uri>,
    prev: Option<Uri>,
    next: Option<Uri>,
}


///     "jsonapi": {
///       "description": "An object describing the server's implementation",
///       "type": "object",
///       "properties": {
///         "version": {
///           "type": "string"
///         },
///         "meta": {
///           "$ref": "#/definitions/meta"
///         }
///       },
///       "additionalProperties": false
///     },
struct JsonAPI {
    version: Option<String>,
    meta: Option<Meta>,
}


///     "error": {
///       "type": "object",
///       "properties": {
///         "id": {
///           "description": "A unique identifier for this particular occurrence of the problem.",
///           "type": "string"
///         },
///         "links": {
///           "$ref": "#/definitions/links"
///         },
///         "status": {
///           "description": "The HTTP status code applicable to this problem, expressed as a string value.",
///           "type": "string"
///         },
///         "code": {
///           "description": "An application-specific error code, expressed as a string value.",
///           "type": "string"
///         },
///         "title": {
///           "description": "A short, human-readable summary of the problem. It **SHOULD NOT** change from occurrence to occurrence of the problem, except for purposes of localization.",
///           "type": "string"
///         },
///         "detail": {
///           "description": "A human-readable explanation specific to this occurrence of the problem.",
///           "type": "string"
///         },
///         "source": {
///           "type": "object",
///           "properties": {
///             "pointer": {
///               "description": "A JSON Pointer [RFC6901] to the associated entity in the request document [e.g. \"/data\" for a primary data object, or \"/data/attributes/title\" for a specific attribute].",
///               "type": "string"
///             },
///             "parameter": {
///               "description": "A string indicating which query parameter caused the error.",
///               "type": "string"
///             }
///           }
///         },
///         "meta": {
///           "$ref": "#/definitions/meta"
///         }
///       },
///       "additionalProperties": false
///     }
struct Error {
    id: Option<String>,
    links: Option<Links>,
    status: Option<String>,
    code: Option<String>,
    title: Option<String>,
    detail: Option<String>,
    source: Option<Source>,
    meta: Option<Meta>,
}

struct Source {
    pointer: Option<JsonPointer>,
    parameter: Option<String>,
}
