package day09

type File struct {
	id, size int
	pos      int
}

func Run(input string) (int, int) {
	mem := make([]int, 0)
	files := make([]File, 0)
	blanks := make([]File, 0)
	ptr := 0
	for i, ch := range input {
		if i%2 == 0 {
			files = append(files, File{id: i / 2, size: int(ch - '0'), pos: ptr})
			ptr += int(ch - '0')
			for range int(ch - '0') {
				mem = append(mem, i/2)
			}
		} else {
			blanks = append(blanks, File{id: -1, size: int(ch - '0'), pos: ptr})
			ptr += int(ch - '0')
			for range int(ch - '0') {
				mem = append(mem, -1)
			}
		}
	}

	bp := 0
	ip := len(mem) - 1
	for true {
		for mem[bp] != -1 {
			bp++
		}
		for mem[ip] == -1 {
			ip--
		}
		if bp >= ip {
			break
		}
		mem[bp], mem[ip] = mem[ip], mem[bp]
		bp++
		ip--
	}

	sum := 0
	for i, v := range mem {
		if v == -1 {
			break
		}
		sum += i * v
	}

	for i := len(files) - 1; i >= 0; i-- {
		for b := range blanks {
			if blanks[b].pos >= files[i].pos {
				break
			}
			if blanks[b].size >= files[i].size {
				files[i].pos = blanks[b].pos
				blanks[b].size -= files[i].size
				blanks[b].pos += files[i].size
				break
			}
		}
	}

	sum2 := 0
	for _, f := range files {
		for i := f.pos; i < f.pos+f.size; i++ {
			sum2 += i * f.id
		}
	}

	return sum, sum2
}
