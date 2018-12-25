const FN = (function () {
  return {
    reduce: (reducer, ls, init) => {
      for (const item of ls) {
        init = reducer(init, item)
      }
      return init
    }
  }
})()
