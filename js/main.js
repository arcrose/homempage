(function() {
  const codeSegment = document.getElementById('diagonal-code-segment')
  const writingSegment = document.getElementById('diagonal-writing-segment')

  const codeSegmentMaxLines = 30
  const codeSegmentTop = [3, 5]            // Start drawing code from x=3, y=5
  const codeSegmentIdentLineDelta = [6, 3] // Add x+5, y+3 to indent the next line of code

  // Produce an HTML <text> element positioned at the given coordinates.
  const text = (x, y, className, text) => {
    let newNode = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    newNode.setAttribute('x', `${x}`)
    newNode.setAttribute('y', `${y}`)
    newNode.textContent = text

    newNode.classList.add(className)

    return newNode
  }

  // Produce an HTML <text> element vertically aligned and trimmed
  // to be displayed in the code segment.
  const code = (line, indent, code) => {
    const startX = indent * codeSegmentIdentLineDelta[0] + codeSegmentTop[0]
    const startY = line * codeSegmentIdentLineDelta[1] + codeSegmentTop[1]

    const charLimit = 80 - (3.15 * line) - (4 * indent)

    return text(startX, startY, 'code', code.slice(0, charLimit))
  }

  // Construct a representation of a state machine that can be operated on
  // to animate text scrolling through the code segment.
  const codeAnimator = (language, source) => ({
    _startLine: 0,
    _linesToShow: 0,
    _language: language,
    _source: source,
  })

  const resetAnimator = ({ language }) => {
    const snippet = pickRandom(CODE_SNIPPETS)
    const source = pickRandom(snippet.sourceFiles)
    return codeAnimator(language, source)
  }

  const _animatorStep = animator => {
    const codeToText = (state, line) => {
      state.nodes.push(code(state.lineNumber, line.indent, line.code))
      return {
        nodes: state.nodes,
        lineNumber: state.lineNumber + 1,
      }
    }
    const linesToShow = Math.min(codeSegmentMaxLines, animator._linesToShow + 1)
    const source = animator._source.linesOfCode.slice(
      animator._startLine,
      animator._startLine + linesToShow)

    if (linesToShow === animator._linesToShow) {
      animator._startLine += 1
    }
    animator._linesToShow = linesToShow
    
    let init = {
      nodes: [],
      lineNumber: 0,
    }

    const { nodes } = FN.reduce(codeToText, source, init)
    return {
      animator,
      nodes,
    }
  }

  const schedule = (initState, loopPause, operations) => ({
    _state: initState,
    _loop: loopPause,
    _ops: operations,
    _index: 0,
  })

  const _run = schedule => {
    setTimeout(() => {
      const op = schedule._ops[schedule._index]
      schedule._state = op(schedule._state)
      schedule._index = (schedule._index + 1) % schedule._ops.length
      _run(schedule)
    }, schedule._loop)
  }

  const drawCode = ({ animator, nodes }) => {
    const { animator: a, nodes: n } = _animatorStep(animator)

    let newNodes = []
    for (let i = 0; i < nodes.length; i++) {
      nodes[i].remove();
      newNodes.push(n[i])
      codeSegment.appendChild(n[i])
    }
    if (n.length > nodes.length) {
      newNodes.push(n[n.length - 1])
      codeSegment.appendChild(n[n.length - 1])
    }

    return {
      animator: a,
      nodes: newNodes,
    }
  }

  const prepareAnimator = ({ animator, nodes }) => {
    if (animator._startLine === animator._source.linesOfCode.length - codeSegmentMaxLines) {
      animator = resetAnimator(animator)
    }
    return {
      animator,
      nodes,
    }
  }

  const pickRandom = (list) => {
    const index = Math.floor((Math.random() * 100000) % list.length)
    return list[index]
  }


  const snippet = pickRandom(CODE_SNIPPETS)
  const source = pickRandom(snippet.sourceFiles)

  let animator = codeAnimator(snippet.languageName, source)
  let state = {
    animator,
    nodes: [],
  }
  let sched = schedule(state, 150, [ drawCode, prepareAnimator ])

  _run(sched)
})()
