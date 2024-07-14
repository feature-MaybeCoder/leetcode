package leetcode

import (
	"fmt"
	"runtime/debug"
	"sort"
	"strconv"
	"unicode"
)


type Atom struct{
    at string
    amount int
} 

func countOfAtoms(formula string) string {
    _, ats := parse(0, formula)

    chars := make([]string,0, len(ats))
    mapped := make(map[string]int, len(ats))

    for _, at := range ats{
        _, ok := mapped[at.at]
        mapped[at.at] += at.amount
        if !ok{
            chars = append(chars, at.at)
        }
    }
    sort.Strings(chars)
    res := ""

    for _, c := range chars{
        res += c
        if mapped[c] > 1{
            res += fmt.Sprintf("%v",mapped[c])
        }
    }

    return res
}

func parse(idx int, formula string)(end int, atoms []Atom){
    atoms = []Atom{}
    currendAtomName := ""
    amount := 1
    i := idx
    for ; i < len(formula); i++{
        char := rune(formula[i])
        if formula[i] == byte('('){
            if len(currendAtomName)>0{
                atoms = append(atoms, Atom{
                    at: currendAtomName,
                    amount: amount,
                })
                currendAtomName = ""
                amount = 1
            }
            j, as := parse(i+1, formula)
            mult := 1
            for k := j; k < len(formula); k++{
                if unicode.IsNumber(rune(formula[k])){
                    parsed, _ := strconv.Atoi(formula[j:k+1])
                    mult = parsed
                    continue
                }
                break
            }
            for idx, _ := range as{
                as[idx].amount *= mult
            }
            atoms = append(atoms, as...)
            i = j-1
            continue
        }
        if formula[i] == byte(')'){
            i++
            break
        }
        if unicode.IsLetter(char) && unicode.IsUpper(char){
            if len(currendAtomName) > 0{
                atoms = append(atoms, Atom{
                    at: currendAtomName,
                    amount: amount,
                })
            }
            amount = 1
            currendAtomName = string(char)
            continue
        }
        if unicode.IsLetter(char) && unicode.IsLower(char){
            currendAtomName += string(char)
            continue
        }
        if unicode.IsNumber(char){
            k := i
            for ; k < len(formula); k++{
                n, err := strconv.Atoi(formula[i:k+1])
                if err != nil{
                    break
                }
                amount = n
            }
            
            i = k - 1
        }
    }
    if len(currendAtomName)>0{
        atoms = append(atoms, Atom{
                    at: currendAtomName,
                    amount: amount,
        })
    }
    
    return i, atoms
}
func init(){
    debug.SetGCPercent(-1)
}