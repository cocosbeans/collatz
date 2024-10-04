function collatzStep(n) {
    if (n % 2 === 0) return n / 2
    else return 3 * n + 1
}

function collatz(n) {
    let x = n
    let nums = []

    while (x >= 2) {
        nums.push(x)
        x = collatzStep(x)
    }

    return nums
}

const first = 100 // First number
const nums = collatz(first)

console.log(nums)
