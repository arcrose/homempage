import Browser
import Date exposing (Date)
import Html exposing (..)
import Html.Attributes exposing (class, id, src, style, type_)
import Html.Events exposing (onClick, onInput)
import Task

main =
  Browser.document
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }


type alias Document msg =
  { title: String
  , body: List (Html msg)
  }


type alias Model =
  { agentStatistics : List AgentStatistic
  , statsToShow : List AgentStatistic
  , currentDay : Date
  , searchText : String
  }


type Msg
  = GotCurrentDay Date
  | GotSearchInput String
  | SortBy (AgentStatistic -> String)


type alias AgentStatistic =
  { operator: String
  , version: String
  , hostName: String
  , hostIdent: String
  , lastHeardFrom: String
  }


type AgentStatColumn
  = Operator
  | Version
  | HostName
  | HostIdent
  | LastSeen

type AgentOnlineStatus
  = Offline
  | Online


type AgentVersionStatus
  = Outdated
  | Recent
  | Latest


init : () -> ( Model, Cmd Msg )
init _ =
  ( { agentStatistics = testData
    , statsToShow = testData
    , currentDay = Date.fromOrdinalDate 0 0
    , searchText = ""
    }
  , Task.perform GotCurrentDay <| Date.today
  )


subscriptions : Model -> Sub Msg
subscriptions _ =
  Sub.none


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
  case msg of
    GotCurrentDay today ->
      ( { model | currentDay = today }
      , Cmd.none
      )

    GotSearchInput input ->
      ( { model
        | searchText = input
        , statsToShow = List.filter (shouldShowStat input) model.agentStatistics
        }
      , Cmd.none
      )

    SortBy field ->
      ( { model
        | statsToShow = List.sortBy field model.statsToShow
        }
      , Cmd.none
      )


view : Model -> Document Msg
view model =
  { title = "MIG Status Page"
  , body =
    [ div [id "main"]
      [ h1 [] [ span [] [ text "MIG Status Page" ] ]
      , viewSearchBar
      , viewAgentStatistics model.currentDay testLatestRelease model.statsToShow
      ]
    ]
  }


viewSearchBar : Html Msg
viewSearchBar =
  div [ id "search" ]
    [ 
    input [ onInput GotSearchInput, type_ "text" ] []
    , img [ class "icon-med", src "img/search.png" ] []
    ]


viewAgentStatistics : Date -> Date -> List AgentStatistic -> Html Msg
viewAgentStatistics today lastRelease stats =
  let
      viewStat stat =
        let
            onlineStatusColor =
              stat
              |> agentActiveStatus today
              |> colorAgentOnlineStatus

            versionStatusColor =
              stat
              |> agentVersionStatus lastRelease
              |> colorVersionStatus
        in
            tr []
              [ td [] [ text stat.operator ]
              , td [ style "background-color" versionStatusColor ] [ text stat.version ]
              , td [] [ text stat.hostName ]
              , td [] [ text stat.hostIdent ]
              , td [ style "background-color" onlineStatusColor ] [ text stat.lastHeardFrom ]
              ]
  in
    table []
      [ thead []
        [ tr []
          [ th [ onClick <| SortBy .operator ]
            [ text "Operator"
            , img [ class "icon-small", src "img/sort_alpha.png" ] []
            ]
          , th [ onClick <| SortBy .version ]
            [ text "Version"
            , img [ class "icon-small", src "img/sort_alpha.png" ] []
            ]
          , th [ onClick <| SortBy .hostName ]
            [ text "Host Name"
            , img [ class "icon-small", src "img/sort_alpha.png" ] []
            ]
          , th [ onClick <| SortBy .hostIdent ]
            [ text "Host OS"
            , img [ class "icon-small", src "img/sort_alpha.png" ] []
            ]
          , th [ onClick <| SortBy .lastHeardFrom ]
          [ text "Last Seen"
            , img [ class "icon-small", src "img/sort_alpha.png" ] []
          ]
          ]
        ]
      , tbody [] <| List.map viewStat stats
      ]


shouldShowStat : String -> AgentStatistic -> Bool
shouldShowStat searchText stat =
  let
      stringified =
        stat.operator ++ stat.version ++ stat.hostName ++ stat.hostIdent ++ stat.lastHeardFrom
  in
      -- If searchText is empty, then `contains` is always `True`.
      String.contains searchText stringified


colorAgentOnlineStatus : AgentOnlineStatus -> String
colorAgentOnlineStatus status =
  case status of
    Offline ->
      "#FA7F7F"

    Online ->
      "#C8FA7F"


colorVersionStatus : AgentVersionStatus -> String
colorVersionStatus status =
  case status of
    Outdated ->
      "#FA7F7F"

    Recent ->
      "#F9B868"

    Latest ->
      "#C8FA7F"


agentActiveStatus : Date -> AgentStatistic -> AgentOnlineStatus
agentActiveStatus today stat =
  let
      date =
        -- Parse a string like "a/b/c _" into a `Maybe (List Int)` containing a, b and c.
        stat.lastHeardFrom
        |> String.split " "
        |> List.head
        |> Maybe.map (String.split "/")
        |> Maybe.map (List.map String.toInt)
        |> Maybe.andThen collect

      appearsOffline =
        case date of
          Just [ y, m, d ] ->
            List.foldl (||) False [ Date.year today > y, Date.monthNumber today > m, Date.day today > (d + 3) ]

          _ ->
            False
  in
      if appearsOffline then
        Offline
      else
        Online


agentVersionStatus : Date -> AgentStatistic -> AgentVersionStatus
agentVersionStatus lastRelease stat =
  let
      date =
        -- Parse a string like "yyyymmdd-_" into a `Maybe (List Int)` containing yyyy, mm and dd.
        stat.version
        |> listApply [ String.slice 0 4, String.slice 4 6, String.slice 6 8 ]
        |> List.map String.toInt
        |> collect

      ( ry, rm, rd ) =
        ( Date.year lastRelease, Date.monthNumber lastRelease, Date.day lastRelease )

      ( appearsOld, appearsOutdated ) =
        case date of
          Just [ y, m, d ] ->
            ( ry > y || Basics.abs (rm - m) >= 6 -- Is the agent at least six months old?
            , ry /= y || rm /= m || rd /= d      -- Is the agent version not the latest?
            )

          _ ->
            ( False, False )
  in
    case ( appearsOld, appearsOutdated ) of
      ( True, _ ) ->
        Outdated
      
      ( _, True ) ->
        Recent

      _ ->
        Latest


listApply : List (a -> b) -> a -> List b
listApply fns x =
  case fns of
    [] ->
      []

    (f::fs) ->
      [ f x ] ++ listApply fs x


collect : List (Maybe a) -> Maybe (List a)
collect ls =
  let
      collector b a =
        case b of
          Just v ->
            Maybe.map (\l -> v :: l) a

          _ ->
            Nothing
  in
      List.foldr collector (Just []) ls


testLatestRelease : Date
testLatestRelease =
  Date.fromOrdinalDate 2018 260


testData : List AgentStatistic
testData =
  [ { operator = "it", version = "20180917", hostName = "test1.testing.com", hostIdent = "CentOS version 7", lastHeardFrom = "2018/09/02 9:25" }
  , { operator = "it", version = "20180827", hostName = "test2.testing.com", hostIdent = "CentOS version 7", lastHeardFrom = "2018/09/07 17:14" }
  , { operator = "it", version = "20180827", hostName = "another1.testing.com", hostIdent = "CentOS version 7", lastHeardFrom = "2018/09/07 17:14" }
  , { operator = "it", version = "20180827", hostName = "another2.testing.com", hostIdent = "CentOS version 7", lastHeardFrom = "2018/08/30 17:00" }
  , { operator = "releng", version = "20180704", hostName = "server1.builder.com", hostIdent = "CentOS version 7", lastHeardFrom = "2018/09/07 17:14" }
  , { operator = "releng", version = "20180704", hostName = "server2.builder.com", hostIdent = "Ubuntu 16.04", lastHeardFrom = "2018/09/07 17:14" }
  , { operator = "security", version = "20180827", hostName = "scanner1.infosec.com", hostIdent = "CentOS 7", lastHeardFrom = "2018/09/07 17:14" }
  , { operator = "security", version = "20160613", hostName = "scanner2.infosec.com", hostIdent = "CentOS 7", lastHeardFrom = "2018/09/07 17:14" }
  , { operator = "security", version = "20180827", hostName = "builder1.infosec.com", hostIdent = "Ubuntu 16.04", lastHeardFrom = "2018/09/07 17:14" }
  , { operator = "testing", version = "20180917", hostName = "builder1.infosec.com", hostIdent = "Ubuntu 16.04", lastHeardFrom = "2018/09/17 13:14" }
  ]
