package interpreter

import (
	"errors"
	"fmt"
	"regexp"
)

// States for the Lexer FSM

type State int

// NOTE - String1 refers to strings in single-quotes ('), and String2 refers to strings in double-quotes (")
// We have two states so that we can have one inside the other.

const (
	ERROR   State = iota
	OPEN    State = iota
	STRING1 State = iota
	STRING2 State = iota
	COMMENT State = iota
	NUMBER  State = iota
	NAME    State = iota
)

/**
 * When lexing a program, certain characters need to be treated as terminators for a particular type of value
 * but should also include that value.  The following instructions dictate what tokens should be added as the
 * FSM moves from character to character through a program.
 */

type Instruction int

const (
	AddNothing         Instruction = iota
	AddToken           Instruction = iota
	AddChar            Instruction = iota
	AddTokenAndChar    Instruction = iota
	AddTokenAndEndSexp Instruction = iota
)

/**
 * The lexer will recurse when S-Expressions are encountered, so that the FSM can essentially be restarted
 * in a nested S-Expression without having to actually start and manage a new FSM.  These directives tell
 * the lexer when it should recurse, return from a recursed lexing, or continue normally.
 */

type RecursiveAction int

const (
	DoNothing RecursiveAction = iota
	Recurse   RecursiveAction = iota
	Return    RecursiveAction = iota
)

/**
 * The following are essentially transition tables for the FSM.
 */

type FSMTransition struct {
	ReadMatch string
	WhatToDo  RecursiveAction
	NextState State
	WhatToAdd Instruction
	NewToken  Token
}

var TransitionsFromOpen = [...]FSMTransition{
	{"\\s", DoNothing, OPEN, AddNothing, NO_TOKEN},
	{"\\(", Recurse, OPEN, AddToken, START_SEXP},
	{"'", DoNothing, STRING1, AddToken, START_STRING},
	{"\"", DoNothing, STRING2, AddToken, START_STRING},
	{";", DoNothing, COMMENT, AddToken, START_COMMENT},
	{"[0-9]", DoNothing, NUMBER, AddTokenAndChar, START_NUMBER},
	{"[0-9a-zA-Z!@#$%^&*-_+=:<,>.?/]", DoNothing, NAME, AddTokenAndChar, START_NAME},
	{"\\)", Return, OPEN, AddToken, END_SEXP},
}

// Handle single-quoted strings
var TransitionsFromString1 = [...]FSMTransition{
	{"\"", DoNothing, STRING1, AddChar, NO_TOKEN},
	{"'", DoNothing, OPEN, AddToken, END_STRING},
	{".", DoNothing, STRING1, AddChar, NO_TOKEN},
}

// Handle double-quoted strings
var TransitionsFromString2 = [...]FSMTransition{
	{"'", DoNothing, STRING2, AddChar, NO_TOKEN},
	{"\"", DoNothing, OPEN, AddToken, END_STRING},
	{".", DoNothing, STRING2, AddChar, NO_TOKEN},
}

var TransitionsFromComment = [...]FSMTransition{
	{"\n", DoNothing, OPEN, AddToken, END_COMMENT},
	{".", DoNothing, COMMENT, AddNothing, NO_TOKEN},
}

var TransitionsFromNumber = [...]FSMTransition{
	{"\\s", DoNothing, OPEN, AddToken, END_NUMBER},
	{"\\)", DoNothing, OPEN, AddTokenAndEndSexp, END_NUMBER},
	{"([0-9]|\\.)", DoNothing, NUMBER, AddChar, NO_TOKEN},
}

var TransitionsFromName = [...]FSMTransition{
	{"\\s", DoNothing, OPEN, AddToken, END_NAME},
	{"\\)", DoNothing, OPEN, AddTokenAndEndSexp, END_NAME},
	{"[0-9a-zA-Z!@#$%^&*-_+=:<,>.?/]", DoNothing, NAME, AddChar, NO_TOKEN},
}

/**
 * Determine what state to transition into based on the current state and the next characters in the program.
 */
func Transition(state State, read string) (error, State, RecursiveAction, []Token) {
	var testTransitions []FSMTransition
	switch state {
	case OPEN:
		testTransitions = TransitionsFromOpen[:]
	case STRING1:
		testTransitions = TransitionsFromString1[:]
	case STRING2:
		testTransitions = TransitionsFromString2[:]
	case COMMENT:
		testTransitions = TransitionsFromComment[:]
	case NUMBER:
		testTransitions = TransitionsFromNumber[:]
	case NAME:
		testTransitions = TransitionsFromName[:]
	}
	for _, transition := range testTransitions {
		matched, err := regexp.MatchString(transition.ReadMatch, read)
		if err != nil {
			return err, state, DoNothing, nil
		} else if matched {
			nextState := transition.NextState
			action := transition.WhatToDo
			var tokens []Token
			switch transition.WhatToAdd {
			case AddNothing:
				tokens = []Token{}
			case AddToken:
				tokens = []Token{transition.NewToken}
			case AddChar:
				tokens = []Token{Token(read)}
			case AddTokenAndEndSexp:
				tokens = []Token{transition.NewToken, END_SEXP}
			case AddTokenAndChar:
				tokens = []Token{transition.NewToken, Token(read)}
			}
			return nil, nextState, action, tokens
		}
	}
	// TODO - Provide useful error descriptions
	errMsg := fmt.Sprintf("No transition from state %d with input %s", state, read)
	return errors.New(errMsg), ERROR, DoNothing, []Token{}
}

/**
 * Lex a program to produce a sequence of tokens and the number of characters read.
 */
func Lex(program string, startIndex int) ([]Token, int) {
	tokens := make([]Token, 0)
	currentState := OPEN
	for i := startIndex; i < len(program); i++ {
		char := string(program[i])
		err, nextState, action, newTokens := Transition(currentState, string(char))
		if err != nil {
			panic(err)
		}
		tokens = append(tokens, newTokens...)
		if action == Recurse {
			nextTokens, newIndex := Lex(program, i+1)
			tokens = append(tokens, nextTokens...)
			i = newIndex
		} else if action == Return {
			return tokens, i
		}
		currentState = nextState
	}
	return tokens, len(program)
}
