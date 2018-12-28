module Authentication
    exposing
        ( SessionToken
        , LoginCredentials
        , RegistrationInfo
        , LoginResponse
        , RegisterResponse
        , LogoutResponse
        , login
        , logout
        , register
        )

{-| Types and API functions for handling presenter authentication and registration.
-}

import Http exposing (Request)
import Json.Decode exposing (Decoder, field, string, maybe)
import Json.Encode as Encode
import Config


{-| Represents a secret session token held by an authenticated user.
-}
type alias SessionToken =
    String


{-| Information required to attempt to login to the server.
-}
type alias LoginCredentials =
    { username : String
    , password : String
    }


{-| Information required to register to the server.
-}
type alias RegistrationInfo =
    { username : String
    , password : String
    , passwordRepeat : String
    }


{-| The body of a response to a login attempt.
-}
type alias LoginResponse =
    { error : Maybe String
    , token : Maybe SessionToken
    }


{-| The body of a response to a registration.
-}
type alias RegisterResponse =
    { error : Maybe String
    , token : Maybe SessionToken
    }


{-| The body of a response to a logout action.
-}
type alias LogoutResponse =
    { error : Maybe String
    }


{-| Construct a request to authenticate the user as a registered presenter.
-}
login : LoginCredentials -> Request LoginResponse
login credentials =
    let
        url =
            "http://" ++ Config.apiServerAddress ++ "/api/presenters/login"

        body =
            credentials
                |> loginRequest
                |> Http.jsonBody
    in
        Http.post url body loginResponse


{-| Construct a request to register a new presenter account.
-}
register : RegistrationInfo -> Request RegisterResponse
register info =
    let
        url =
            "http://" ++ Config.apiServerAddress ++ "/api/presenters/register"

        body =
            info
                |> registerRequest
                |> Http.jsonBody
    in
        Http.post url body registerResponse


{-| Construct a new request to terminate the session of an authenticated presenter.
-}
logout : SessionToken -> Request LogoutResponse
logout token =
    let
        url =
            "http://" ++ Config.apiServerAddress ++ "/api/presenters/logout"

        body =
            token
                |> logoutRequest
                |> Http.jsonBody
    in
        Http.post url body logoutResponse


loginRequest : LoginCredentials -> Encode.Value
loginRequest { username, password } =
    Encode.object
        [ ( "username", Encode.string username )
        , ( "password", Encode.string password )
        ]


registerRequest : RegistrationInfo -> Encode.Value
registerRequest { username, password, passwordRepeat } =
    Encode.object
        [ ( "username", Encode.string username )
        , ( "password", Encode.string password )
        ]


logoutRequest : SessionToken -> Encode.Value
logoutRequest token =
    Encode.object
        [ ( "sessionToken", Encode.string token )
        ]


loginResponse : Decoder LoginResponse
loginResponse =
    Json.Decode.map2 LoginResponse
        (field "error" (maybe string))
        (field "sessionToken" (maybe string))


registerResponse : Decoder RegisterResponse
registerResponse =
    Json.Decode.map2 RegisterResponse
        (field "error" (maybe string))
        (field "sessionToken" (maybe string))


logoutResponse : Decoder LogoutResponse
logoutResponse =
    Json.Decode.map LogoutResponse
        (field "error" (maybe string))
