package get_watched_videos_by_your_friends

import (
	"sort"
)
func watchedVideosByFriends(watchedVideos [][]string, friends [][]int, id int, level int) []string {
	qLen := len(friends)
	queue := make([]int, 0, qLen)
	visited := make([]bool, qLen)
	queue = append(queue, id)
	visited[id] = true
    for l := 0; l < level; l++ {
		next := make([]int, 0, qLen)
		for _, node := range queue{
			for _, neighb := range friends[node]{
				if visited[neighb]{
					continue
				}
				visited[neighb] = true
				next = append(next, neighb)
			}
		}
		queue = next;
		if len(next) == 0{
			break
		}
		
	}
	
	res := make([]string, 0, len(queue))
	if len(queue) ==0{
		return res
	}
	freq := make(map[string]int, len(queue))
	for _, friend := range queue{
		for _, film := range watchedVideos[friend]{
			_, contains := freq[film]
			if contains{
				freq[film] +=1
				continue
			}
			res = append(res, film)
			freq[film] = 1
		}
	}
	sort.Slice(res, func (i, j int) bool {
		left_f := freq[res[i]]
		right_f := freq[res[j]]
		if left_f == right_f{
			return res[i] < res[j]
		}
		return left_f < right_f
	})
    return res
}