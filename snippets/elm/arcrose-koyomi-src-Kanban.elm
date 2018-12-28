module Kanban exposing ( Kanban, Msg, empty, update, view )

import Html exposing (..)
import Html.Attributes exposing (style)
import Html.Events exposing (onClick)


type Msg
  = AddCard ColumnIndex Card
  | MoveCard ColumnIndex CardIndex Direction
  | AddColumn String

type alias CardIndex = Int

type alias ColumnIndex = Int

type Direction
  = Up
  | Down
  | Left
  | Right

type alias Card =
  { title: String
  , description: String
  , index: CardIndex
  }

type alias Column =
  { title: String
  , cards: List Card
  , index: ColumnIndex
  }

type alias Kanban =
  { columns: List Column
  }

empty : Kanban
empty =
  { columns =
    [ { title = "Test 1"
      , index = 0
      , cards =
        [ { title = "Card 1 1"
          , description = "First ever card!"
          , index = 0
          }
        , { title = "Card 1 2"
          , description = "Second card"
          , index = 1
          }
        ]
      }
    , { title = "Test 2"
      , index = 1
      , cards =
        [ { title = "Card 2 1"
          , description = "Card in second column"
          , index = 0
          }
        ]
      }
    , { title = "Test 3"
      , index = 2
      , cards =
        [ { title = "Card 3 1"
          , description = "Card in third column"
          , index = 0
          }
        , { title = "Card 3 2"
          , description = "Another card"
          , index = 1
          }
        , { title = "Card 3 3"
          , description = "Last card"
          , index = 2
          }
        ]
      }
    ]
  }

update : Msg -> Kanban -> ( Kanban, Cmd Msg )
update msg kanban =
  case msg of
    AddCard col card ->
      ( addCard kanban col card
      , Cmd.none
      )

    MoveCard col card dir ->
      ( moveCard kanban col card dir
      , Cmd.none
      )

    AddColumn title ->
      ( addColumn kanban title
      , Cmd.none
      )

addCard : Kanban -> ColumnIndex -> Card -> Kanban
addCard board col card =
  let
    addCardToColumn columns colIndex newCard =
      let
        column =
          columns
            |> List.head
            |> Maybe.withDefault { title = "new column", index = 0, cards = [] }

        remainingColumns =
          columns
            |> List.tail
            |> Maybe.withDefault []

        newColumn =
          { column | cards = List.append column.cards [ newCard ] }
      in
        if colIndex == 0 then
          newColumn :: remainingColumns
        else
          column :: (addCardToColumn remainingColumns (colIndex - 1) newCard)
  in
    { board | columns = (addCardToColumn board.columns col card) }

moveCard : Kanban -> ColumnIndex -> CardIndex -> Direction -> Kanban
moveCard board col card dir =
  case dir of
    Up ->
      board

    Down ->
      board
    
    Left ->
      if col <= 0 then
        board
      else
        moveCardLeft board col card

    Right ->
      if col >= List.length board.columns then
        board
      else
        moveCardRight board col card

moveCardLeft : Kanban -> ColumnIndex -> CardIndex -> Kanban
moveCardLeft board columnIndex cardIndex =
  board

moveCardRight : Kanban -> ColumnIndex -> CardIndex -> Kanban
moveCardRight board columnIndex cardIndex =
  board

addColumn : Kanban -> String -> Kanban
addColumn board title =
  let
    newColumn =
      { title = title
      , cards = []
      , index = List.length board.columns
      }
  in
    { board
    | columns = List.append board.columns [ newColumn ]
    }

view : Kanban -> Html Msg
view board =
  let
    addColBtn =
      button
        (List.append [ onClick (AddColumn "new column") ] newColBtnStyle)
        [ text "Add column" ]

    columns =
      List.map viewColumn board.columns
  in
    div []
      [ addColBtn
      , div (boardStyle board) columns
      ]

viewColumn : Column -> Html Msg
viewColumn col =
  let
    header =
      [ p [] [ text col.title ]
      , hr [] []
      ]

    cards =
      List.map viewCard col.cards

    newCard =
      { title = "new card"
      , description = "auto generated"
      , index = List.length col.cards
      }

    addCardBtn =
      button
        (List.append
          [ onClick ( AddCard col.index newCard ) ]
          newCardBtnStyle
        )
        [ text "+" ]

    content =
      List.foldr List.append []
        [ header
        , cards
        , [ addCardBtn ]
        ]
  in
  div columnStyle content

viewCard : Card -> Html Msg
viewCard card =
  div cardStyle
    [ p [ noMargin ] [ text card.title ]
    , p [ noMargin ] [ text card.description ]
    , footer []
      [ button [ onClick (MoveCard Left) ] [ text "L" ]
      , button [ onClick (MoveCard Right) ] [ text "R" ]
      ]
    ]

boardStyle : Kanban -> List (Html.Attribute Msg)
boardStyle board =
  let
    gridTemplateColumns =
      board.columns
      |> List.length
      |> (\x -> List.repeat x "1fr")
      |> String.join " "

    gridTemplateRows =
      board.columns
      |> List.map (\col -> List.length col.cards)
      |> List.maximum
      |> Maybe.withDefault 0
      |> (\x -> List.repeat x "1fr")
      |> String.join " "
  in
    [ style "display" "grid"
    , style "grid-template-columns" gridTemplateColumns 
    , style "grid-template-rows" gridTemplateRows
    , style "grid-column-gap" "8%"
    , style "height" "100%"
    ]

newColBtnStyle : List (Html.Attribute Msg)
newColBtnStyle =
  []

newCardBtnStyle : List (Html.Attribute Msg)
newCardBtnStyle =
  []

columnStyle : List (Html.Attribute Msg)
columnStyle =
  [
  ]

cardStyle : List (Html.Attribute Msg)
cardStyle =
  [ style "border-radius" "4px"
  , style "background-color" "#82BBC2"
  , style "padding" "8px 16px"
  , style "margin-bottom" "16px"
  , style "width" "90%"
  ]

noMargin : Html.Attribute Msg
noMargin =
  style "margin" "0"