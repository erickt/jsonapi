type Object = serde_json::Value;

/// http://jsonapi.org/format/#document-top-level
///
/// A JSON object **MUST** be at the root of every JSON API request and response containing data. This
/// object defines a document's "top level".
///
/// A document **MUST** contain at least one of the following top-level members:
///
/// * data: the document's "primary data"
/// * errors: an array of error objects
/// * meta: a meta object that contains non-standard meta-information.
///
/// The members data and errors **MUST** NOT coexist in the same document.
///
/// A document **MAY** contain any of these top-level members:
///
/// * jsonapi: an object describing the server's implementation
/// * links: a links object related to the primary data.
/// * included: an array of resource objects that are related to the primary data and/or each other
///   ("included resources").
///
/// If a document does not contain a top-level data key, the included member **MUST** NOT be present
/// either.
enum Document {
    Data {
        jsonapi: Option<JsonAPIObject>,
        data: PrimaryData,
        meta: Option<Meta>,
        links: Links,
        included: Vec<T>,
    }
    Error {
        jsonapi: Option<String>,
        errors: Vec<Error>,
        meta: Option<Meta>,
        links: Links,
    }
}



/// The top-level links object **MAY** contain the following members:
///
/// * self: the link that generated the current response document.
/// * related: a related resource link when the primary data represents a resource relationship.
///   pagination links for the primary data.
struct TopLevelLinks {
    self_: Option<Link>,
    related: Option<Link>,
    links: Links,
}


/// The document's "primary data" is a representation of the resource or collection of resources
/// targeted by a request.
///
/// Primary data **MUST** be either:
///
/// * a single resource object, a single resource identifier object, or null, for requests that
///   target single resources
/// * an array of resource objects, an array of resource identifier objects, or an empty array ([]),
///   for requests that target resource collections
///
enum PrimaryData {
    Null,
    Single(ResourceObject),
    SingleIdentifier(ResourceIdentifierObject),
    Array(Vec<ResourceObject>),
    ArrayIdentifier(Vec<ResourceIdentifierObject>),
}


/// http://jsonapi.org/format/#document-resource-objects
///
/// "Resource objects" appear in a JSON API document to represent resources.
///
/// A resource object **MUST** contain at least the following top-level members:
///
/// * id
/// * type
///
/// Exception: The id member is not required when the resource object originates at the client and
/// represents a new resource to be created on the server.
///
/// In addition, a resource object **MAY** contain any of these top-level members:
///
/// * attributes: an attributes object representing some of the resource's data.
/// * relationships: a relationships object describing relationships between the resource and other
///   JSON API resources.
/// * links: a links object containing links related to the resource.
/// * meta: a meta object containing non-standard meta-information about a resource that can not be
///   represented as an attribute or relationship.
struct ResourceObject {
    id: Id,
    type_: Type,
    attributes: Option<Attributes>,
    relationships: Option<Relationships>,
    links: Option<Links>,
    meta: Option<Meta>,

    /// data: Object,
}


/// http://jsonapi.org/format/#document-resource-object-identification
struct Id(Name);


/// http://jsonapi.org/format/#document-resource-object-identification
struct Type(Name);


/// http://jsonapi.org/format/#document-resource-object-attributes
///
/// The value of the attributes key **MUST** be an object (an "attributes object"). Members of the
/// attributes object ("attributes") represent information about the resource object in which it's
/// defined.
///
/// Attributes may contain any valid JSON value.
///
/// Complex data structures involving JSON objects and arrays are allowed as attribute values.
/// However, any object that constitutes or is contained in an attribute **MUST** NOT contain a
/// relationships or links member, as those members are reserved by this specification for future
/// use.
///
/// Although has-one foreign keys (e.g. author_id) are often stored internally alongside other
/// information to be represented in a resource object, these keys SHOULD NOT appear as attributes.
struct Attributes(Object);


/// http://jsonapi.org/format/#document-resource-object-relationships
/// The value of the relationships key **MUST** be an object (a "relationships object"). Members of the
/// relationships object ("relationships") represent references from the resource object in which
/// it's defined to other resource objects.
///
/// Relationships may be to-one or to-many.
///
/// A "relationship object" **MUST** contain at least one of the following:
///
/// * links: a links object containing at least one of the following:
///   * self: a link for the relationship itself (a "relationship link"). This link allows the
///     client to directly manipulate the relationship. For example, it would allow a client to
///     remove an author from an article without deleting the people resource itself.
///   * related: a related resource link
/// * data: resource linkage
/// * meta: a meta object that contains non-standard meta-information about the relationship.
///
/// A relationship object that represents a to-many relationship **MAY** also contain pagination links
/// under the links member, as described below.
struct Relationships {
    links: Option<Links>,
    data: Option<ResourceLinkage>,
    meta: Option<Meta>,
}


/// http://jsonapi.org/format/#document-resource-object-related-resource-links
///
/// A "related resource link" provides access to resource objects linked in a relationship. When
/// fetched, the related resource object(s) are returned as the response's primary data.
///
/// For example, an article's comments relationship could specify a link that returns a collection
/// of comment resource objects when retrieved through a GET request.
///
/// If present, a related resource link **MUST** reference a valid URL, even if the relationship isn't
/// currently associated with any target resources. Additionally, a related resource link **MUST** NOT
/// change because its relationship's content changes.
struct RelatedResourceLink { ... }


/// http://jsonapi.org/format/#document-resource-object-linkage
///
/// Resource linkage in a `compound document` allows a client to link together all of the included
/// resource objects without having to GET any URLs via `links`.
///
/// Resource linkage **MUST** be represented as one of the following:
///
/// * null for empty to-one relationships.
/// * an empty array ([]) for empty to-many relationships.
/// * a single `resource identifier object` for non-empty to-one relationships.
/// * an array of `resource identifier objects` for non-empty to-many relationships.
enum ResourceLinkage {
    Null,
    Single(ResourceIdentifierObject),,
    Array(Vec<ResourceIdentifierObject>),
}


/// http://jsonapi.org/format/#document-resource-object-links
///
/// The optional links member within each resource object contains links related to the resource.
///
/// If present, this links object **MAY** contain a self link that identifies the resource represented by
/// the resource object.
struct ResourceLinks(Links);

impl ResourceLinks {
    fn self_(&self) -> Option<&Link> {
        links.find("self")
    }
}


/// http://jsonapi.org/format/#document-resource-identifier-objects
///
/// A "resource identifier object" is an object that identifies an individual resource.
///
/// A "resource identifier object" **MUST** contain `type` and `id` members.
///
/// A "resource identifier object" **MAY** also include a meta member, whose value is a `meta` object
/// that contains non-standard meta-information.
struct ResourceIdentifierObject {
    /// A "resource identifier object" **MUST** contain type and id members.
    id: Id,
    type_: Type,

    /// A "resource identifier object" **MAY** also include a meta member, whose value is a meta object
    /// that contains non-standard meta-information.
    meta: Option<Meta>,
}


/// http://jsonapi.org/format/#document-compound-documents
///
/// To reduce the number of HTTP requests, servers **MAY** allow responses that include related
/// resources along with the requested primary resources. Such responses are called "compound
/// documents".
///
/// In a compound document, all included resources **MUST** be represented as an array of resource
/// objects in a top-level `included` member.
///
/// Compound documents require "full linkage", meaning that every included resource **MUST** be
/// identified by at least one `resource identifier object` in the same document. These resource
/// identifier objects could either be primary data or represent resource linkage contained within
/// primary or included resources. The only exception to the full linkage requirement is when
/// relationship fields that would otherwise contain linkage data are excluded via `sparse
/// fieldsets`.
struct CompoundDocuments { ... }


/// http://jsonapi.org/format/#document-meta
///
/// Where specified, a meta member can be used to include non-standard meta-information. The value
/// of each meta member **MUST** be an object (a "meta object").
///
/// Any members **MAY** be specified within meta objects.
struct Meta(Object);


/// http://jsonapi.org/format/#document-links
///
/// Where specified, a links member can be used to represent links. The value of each links member
/// **MUST** be an object (a "links object").
///
/// Each member of a links object is a "link". A link **MUST** be represented as either:
///
/// * a string containing the link's URL.
/// * an object ("link object") which can contain the following members:
///   * href: a string containing the link's URL.
///   * meta: a meta object containing non-standard meta-information about the link.
///
struct Links(BTreeMap<Name, Link>);

enum Link {
    URL(URL),
    Object {
        href: URL,
        meta: Meta,
    },
}

struct URL { ... }


/// http://jsonapi.org/format/#document-jsonapi-object
///
/// A JSON API document **MAY** include information about its implementation under a top level
/// `jsonapi` member. If present, the value of the `jsonapi` member **MUST** be an object (a
/// "jsonapi object"). The jsonapi object **MAY** contain a `version` member whose value is a
/// string indicating the highest JSON API version supported. This object **MAY** also contain a
/// `meta` member, whose value is a `meta` object that contains non-standard meta-information.
struct JsonAPIObject {
    version: Option<String>,
    meta: Option<Meta>,
}


/// http://jsonapi.org/format/#document-member-names
///
/// All member names used in a JSON API document **MUST** be treated as case sensitive by clients and
/// servers, and they **MUST** meet all of the following conditions:
///
/// * Member names **MUST** contain at least one character.
/// * Member names **MUST** contain only the allowed characters listed below.
/// * Member names **MUST** start and end with a "globally allowed character", as defined below.
///
/// To enable an easy mapping of member names to URLs, it is **RECOMMENDED** that member names use
/// only non-reserved, URL safe characters specified in RFC 3986.
///
/// # Allowed Characters
///
/// The following "globally allowed characters" **MAY** be used anywhere in a member name:
///
/// * U+0061 to U+007A, "a-z"
/// * U+0041 to U+005A, "A-Z"
/// * U+0030 to U+0039, "0-9"
/// * any UNICODE character except U+0000 to U+007F (not recommended, not URL safe)
///
/// Additionally, the following characters are allowed in member names, except as the first or last
/// character:
///
/// * U+002D HYPHEN-MINUS, "-"
/// * U+005F LOW LINE, "_"
/// * U+0020 SPACE, " " (not recommended, not URL safe)
///
/// # Reserved Characters
///
/// The following characters **MUST NOT** be used in member names:
///
/// * U+002B PLUS SIGN, "+" (used for ordering)
/// * U+002C COMMA, "," (used as a separator between relationship paths)
/// * U+002E PERIOD, "." (used as a separator within relationship paths)
/// * U+005B LEFT SQUARE BRACKET, "[" (used in sparse fieldsets)
/// * U+005D RIGHT SQUARE BRACKET, "]" (used in sparse fieldsets)
/// * U+0021 EXCLAMATION MARK, "!"
/// * U+0022 QUOTATION MARK, '"'
/// * U+0023 NUMBER SIGN, "#"
/// * U+0024 DOLLAR SIGN, "$"
/// * U+0025 PERCENT SIGN, "%"
/// * U+0026 AMPERSAND, "&"
/// * U+0027 APOSTROPHE, "'"
/// * U+0028 LEFT PARENTHESIS, "("
/// * U+0029 RIGHT PARENTHESIS, ")"
/// * U+002A ASTERISK, "*"
/// * U+002F SOLIDUS, "/"
/// * U+003A COLON, ":"
/// * U+003B SEMICOLON, ";"
/// * U+003C LESS-THAN SIGN, "<"
/// * U+003D EQUALS SIGN, "="
/// * U+003E GREATER-THAN SIGN, ">"
/// * U+003F QUESTION MARK, "?"
/// * U+0040 COMMERCIAL AT, "@"
/// * U+005C REVERSE SOLIDUS, "\"
/// * U+005E CIRCUMFLEX ACCENT, "^"
/// * U+0060 GRAVE ACCENT, "`"
/// * U+007B LEFT CURLY BRACKET, "{"
/// * U+007C VERTICAL LINE, "|"
/// * U+007D RIGHT CURLY BRACKET, "}"
/// * U+007E TILDE, "~"
struct Name(String);


//////////////////////////////////////////////////////////////////////////////


