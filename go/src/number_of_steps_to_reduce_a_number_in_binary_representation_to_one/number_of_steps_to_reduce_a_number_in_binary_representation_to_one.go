package leetcode

func numSteps(s string) int {
	one := byte('1')
	bitsLen := len(s)
	bitsBound := bitsLen - 1
	bits := make([]int, bitsLen)
	for i := bitsLen - 1; i >= 0; i-- {
		if s[i] == one {
			bits[bitsBound-i] = 1
			continue
		}
		bits[bitsBound-i] = 0
	}
	steps := 0
	for bitsLen != 1 {
		steps += 1
		if bits[0] == 0 {
			devide(&bits)
			bitsLen -= 1
		} else {
			bitsLen += add(&bits)
		}
		if steps > 10 {
			break
		}
	}
	return steps
}
func devide(bits *[]int) {
	for i := 0; i < len(*bits)-1; i++ {
		(*bits)[i] = (*bits)[i+1]
	}
	*bits = (*bits)[0 : len(*bits)-1]
}
func add(bits *[]int) (shiftAmout int) {

	for i := 0; i < len(*bits); i++ {
		if (*bits)[i] == 0 {
			(*bits)[i] = 1
			return 0
		}
		(*bits)[i] = 0
	}
	*bits = append(*bits, 1)
	return 1
}
