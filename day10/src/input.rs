pub const example_input: &str = r".....
.S-7.
.|.|.
.L-J.
.....";

pub const input: &str = r"|-F7|77FJ-J7..L.-7-7.F--7-J7-FJ7|.FFFF7F-|7|7.J.-|7--F|-LL---77FF--7-|-LF77F|7FF-JFF7FJ-|7F.F7FFL-J7FFL-J-7-FFF7JF|-L77|-F7.FL7F7-7.7.FLF-F7
7.L|.LJJ.LLL7L-LJ|L--J7FL|LF7.LJL77.7--J7LFL|.L.L7JJF7JJ|JFLFFJ7|J|F-LF-||FF|7F7|-7.7-J7|FJ.|L7--7J-L7||J--.||LJ.FLJ.-LJL-JJ7JF7L7|L7F77L777
--FL7FJ-7.||J.L||LFL-|--7L7L-7|LF77.|L|.7-|JL7F7J|...L7FJ77--7|||F-|J7LL|L-7F7|LJ.|LJ|.L-FJ.FFLJF|JL7|FL-L|FL|.FFJL-7J.LF|-L--777LFJL|LJ-|J7
FL|--JJJ|7-7F--JJ-7.|L7.|FL-F|FF||F-7.LF.L7.|LLJF77FJ|F|7|.7FLFF-J.7-7-LL7FJ||7.LFLJ7FJ.7JL7-7|--JLFL-L-7-F|-JF777F-J||--JF77.JL7-|.FJF|.LJ-
-7JFJ|--J|L7JF7-|J|-FLJ7--LF7F--J||FJF7JLFJFJFJFFJ|7|LF7JLLJJ|FJ7J||LF.F-J|FJL77F||FF7|F777|JLF7...|L7FF-7.F-|JJ.FL..LJ-JF|L7L7F-7F--J-L-7J.
|JF-JL7LF|.|.-7LJ-F-7|.|7FF||L7F-J||FJL7J|---L77J.||FJ||L|LJJ|L|L-777F7L7FJL-7L7F777|L7F---7J.|J-FF7-J--JLF|J.|FJ--|-.L-FF7.JFLJ7|J.L7J.|||F
LF7-F-|-||7JF7LFJ7||.FFL-LL||.||F7|LJF-JL7FL-7F7.|F-JFJ|777|.|-F77LF-J|FJL-7-L7||L7-L7|L-7FJ-FF-F7JLFJL|7F7|L-L7J7-J.|..---JFLJ|F7F7.|77||7J
L|LF|7L-LFJ.||7LLF|7-JJ7JJ.|L-J|||L7FJ|F77J7LFJL777-F|FJ||-77LF--77L-7|L7F-JF-J||FJF-JL7J|||-FF7F7.|..F.LL.L-F|J-L7-777-|LL|FJ-FJ7|F--77FJLJ
.7-L77.7F|LF-FJ..JL7-|.77FFL--7||L-JL7F|L77F-|F-JF77FJ|FJLF||-L7FJ-F-JL7|||JL-7||L7L--7L7||F7FJ||L-77.|-FJFL7..|||JFJJJF7|.FJ|..FFJJ7F-|J.7.
LLL|L7--7-.|J|LFJ-FJ.L-7-F-7F7|||F7F7L7L7L77FJL--J|-L7L7JF77J-FJL-7L--7|||F7F7|||FJ-F7|FJ|LJ|L7||F-J|FL-|7|L77FJJ|FF|7--7-F|-JL-F.|JL7---L-7
LFL-7J-||FF|||-J.L|.J7LL-L7|||||||LJL-JF|FJFJF----J-LL7|7|.F7JL-7FJF--JLJ||LJ||LJL-7||||FJF-JFJLJL7-J7J.77J.|-||F|FFJ-FJL7L|7||7L-|-F|J|-|-L
FJ7LL-.|.--FL7|JF7J7L77LF-JLJLJ||L---7F-J|FJFJF7-F7.FFJL7F7|L-7L|L7|F--7FJL-7||F---J|LJLJFJF7L7F7FJJ.L.F|FF|-7|-7LF-.FL7.LJ.L-LF.L|F|JLL-J-|
F--7JFFL.|LLLJ7F||||JFLFJF---7FJ|F7F-JL-7|L7|-|||||F7|F-J||L-7L7L7|LJ.FJ|F7FJLJL7-F7L7F--JL||FJ|LJ7.|.F7F7J|LLF7L7F77-|FL-LJ---7F|LFL7F|||.|
|LLJ7|J7-|F7JL|FLLJ|FLLL-JF--JL7||||F-7FJ|FJL7||FJ||LJ|F7||.LL7|FJL7F7L7LJLJF7F7|FJL7||F7FFJ|L7L7-FF-7||||LJ-.LL.F--7JL7.FF.7|.F-J||LJ.|L7J7
L7LLLL7||LJJ-.|FL||LFJLF7FJF7F7|||LJL7||FJL7FJ|||FJL-7||||L7F-J|L-7||L7L-7F-J||LJL-7|||||FJFJFJFJF7|FJ|||L77FF7L.L7FJF||-7.F-7-J7|LJ7-7-|J.F
FL-777-|7F-|FLLF7FF7FF7||L-JLJ|||L-7FJ||L7FJL-J||L7F7|||LJFJL-7L--JLJFJF-JL-7|L-7F7|LJLJ|L7L7|FJL|||L7|||FJF7LF-F-J|F7F|7L777|LJF-JJL-77|L-.
7J--7-J.|LF|7.|L|7.LFJLJ|FF7|FJLJF-J|FJL7|L---7||FJ|LJLJF-J.F-JF7F7F7L7L7F--JL--J|LJF7F7L7|FJ|L-7|||FJ|||L-J|F7-L-7||L7F-7F77|7.L-7J.F|LLFL7
|LJJ.|.F|L-J7-77LLFJL--7L-JL-JF-7|F7||JFJL-7F7||||LL7F--J-F7L-7|||LJ|FJFJL--7F--7L--JLJL7||L-JF7LJLJL7|LJF7FJ||JF7|LJFJL7LJL7LF-.F7JL-77.|J7
FL7|F7F-7-|7F7F77FF7LF7L-----7L7|||LJL7L-7FJ||||||F-JL7F--JL7-LJLJF-JL-JF7F-J|F-JF-7F--7|||F--J|F7F--JL7FJLJFJL7||L7FJF7L7F-JF|LFL|7|F|L-FF7
JL7-J.F7|F-LJFJL7FJ|FJL7F---7L-J||L--7L-7|L7|LJLJLJF7FJ|F---JF--7FL----7|LJ.FJL7LL7||F7LJLJL--7||LJF7F7||F7JL-7|||FJ|FJL-JL---7.F-JLJ-|JFJ||
|-F7F|LJ7.|..L7FJ|FJL-7LJF-7L---JL7F-JF-JL7|L-7F---JLJFJL---7|F7L7F7F7FJL-7FJF-J7F||LJL--7F---JLJ|FJLJLJLJL7F7||||L7LJF--7F7F7|77.FJ|LJ.LJ-J
.L-J||7|.-7-F-JL-JL7F7L-7L7L-----7|L7FJFF7||F-JL--7FF7L7F---JLJL7||LJ|L-7FJL7L--7FJ||F7FFJL7F7F7F7L7F-7F---J|LJLJ|JL-7L-7LJLJLJ-J-LL|7J.|L77
||L7F7JL--|FL-----7|||F7L-JF-----J|FJL-7|LJ|L-7F--JFJL7|L7F7F-7J||L-7|F7||F7|F--JL7|FJL7L7FJ|LJ|||FJL7LJF-7LL-7F-JF7FJF7L7F---7J|7.LJF--LJLF
L-JL7.L7|.FF7F----J||||L--7|F7F--7|L7F-JL7FJF7||LF7|F-J|FJ|||FJFJ|F-JLJLJLJ|||F77FJ|L-7L-JL7L7FJ|LJF7|F7L7L7F7|L7FJ||FJL7||F--J-J7FJ.7F|.7.L
F7FL--|-7.FJ|L7F--7LJ|L-7FJLJLJF-J|FJL-7F|L7|||L-JLJL7FJL7||||FJFJL-----7F7||LJL7L7|F7L---7L7||FJF7|LJ|L7|FJ|||FJL7||L7FJLJL--7.LF7JFF-.L-7.
LJ-|L-L-J-L7L7LJF-JF7L-7||F7F77L-7|L--7L-JFJ|||F--7F-JL-7||||||FJ-F7LF7|||LJL-7FJFJ||L7F--JFJ|||FJLJF7L7|||FJ|||F7|||FJL------JF7FF-LJLFLLJF
.|F|.LJ-7|-L7|F7L--JL-7LJ||||L-7FJL7F7L--7|.|||L7-||F7F7|||LJLJL7FJ|FJ|FJL7F7FJ|FJFJL7|L--7L-J|||F7.||FJLJ||FJ|||LJLJL7-F-77F7F|L-7FJ7FJ.LF|
F-J|77|.F7-FJLJ||F7-F7L-7|||L-7||F-J||F--JL7||L7L7LJ|||||||F----JL7|L7|L-7|||L7|L7|F7||F7-|F--J|LJL7|||F--J|L-JLJF-7F7L7L7|FJL7|F-JJF77-|.LL
FJL|.FL7FJ|L--7L-JL-JL7FJ|||F-JLJL7FJ||F-7FJ||FJFJF7|LJ|||||F7|F7FJL7||F-J|||7||FJLJLJLJL7|||F7|F--J||||-F7L7F-7FJ.LJL7L-JLJF7LJL-7-JLL7L--J
LL7FL7-7J-|JF7L------7||FJ||L----7||FJLJFJL7||L7|FJ|L-7||LJ||L7||L-7LJ|L7FJ||FJ|L---7F7F-J|L7||||F7J||||FJL7|L7LJF7F-7L7F7F-JL-7F-J-J-|7J.F|
F.L77|J|.LFFJL----7FFJLJL-J|F7FF-JLJ|F-7L7FJ|L-J||FJ7FJLJF-J|FJ|L77L-7|FJL7|LJFJ.F-7LJ|L7.L7||LJ|||FJLJ|L7FJ|FJF-J||FJJ|||L---7||LJ-L---77FJ
|F-|-|FJ7F-JF7F--7L-JF----7||L7L-7F-J|FJFJL7L--7LJ|F7L--7L7FJL-JFJF7FJ||F-J|F7|F7|FJF7L7L-7||L7FJ|||F--JFJ|FJL-JF7LJL-7LJL7F--JLJ.J7-7J...|7
F-7LFJ-.LL--JLJF-JF7FJF---J|L7|F-J|F7||FJF-JFF7|F-J||-F7|FJ|F---JFJ|L7|||F7||LJ||||FJL-JF-J||FJL7|LJ|F-7|FJ|F---J|F7F7|FF7LJF7F7|7||F7J|.FF-
--7|L7.|-LLF7F7L-7|LJFJF7F7L-JLJF7|||||L7L-7FJ|||F7||FJ||L7|L7F-7L7L7|||||||L7FJLJ|L---7|F7|LJF-JL-7||FJ|L-JL-7F7LJ|||L7|L-7|||L77.LFF7J-7JJ
|FJFJ.-J7.L||||F7LJF7L7|LJL-----J||||||7|F-JL7|||||||L7||FJ|FJ|FJ7L7|||||||L7|L-7FJF7F7||||L7FJFF7|||||F|F----J||F7LJL-J|F-J|||FJF7||||.L|.7
7JL7.-...F7|||||L7FJL-J|F----7F7FJLJ|||FJ|F7FJ||||||L7||||FJL7|L7F7||||||||FJL7FJ|FJLJ||||L7|L7FJL7|||L7|L7F---JLJL-7F--JL7L|||L7||F-JL7JL--
JL-7J7--7|LJLJ||FJL--7FJ|F7F7LJLJF-7LJLJ-LJ|L7|||||L7||LJ|L7FJ|FJ|LJ||||||LJ-FJL7LJF7F||||FJ|FJL7FJ||L7||FJ|F------7|L-7F7L-JLJFJ|LJF7FJ|7||
.LLJL7L|FJF--7LJ|F7|FJL7LJLJL7F7FJJL----7F-JFJ|||||FJ||F-JFJL-JL7L-7||||LJF--JF7L--JL7LJ|||FJ|F7|L7|L-JLJL-JL-7F7F7LJF7LJ|F----J.|F-JLJJ--L7
-.|.FL-FL7|F7L-7LJL-JF-JFF7F7LJLJF7F7F--JL--JFJ|LJ|L7|||F7L-7F--JF7|||LJF-JF-7|L7F---J|FJ||L7LJ|L7||F---7F-7F-J|LJL-7||F7||F7F7F7||FF-7-FJ7|
|LF77-FLFJ||L--JF7F--J7F7|LJL----JLJLJF7F7F-7L7L-7L7||||||F7|L7F7|LJ||F-JF7|FJ|FJ|F7LF-JFJ|FJF-JFJ||L7F7||LLJF7|F---J|||LJLJLJLJLJL7|FJ-7-JJ
7.L77L-FL-JL-7F7|LJ7F--JLJF7F------7F-JLJLJFJFJF7L7LJLJ||||||J|||L-7LJ|F-J|||FJL7||L7|F7L7|L7L7FJFJL7LJ|LJF--JLJL--7FJLJF7F7F------J|L-7J7J.
|7|LL7.7JL.F-J|||F-7L---7FJLJF-----J|F---7FJLL7|L7L--7L||LJLJFJ|L7FJF7||F-J|||F-JLJFJ||L7||FJL|L7|F-JF-JF7|F-------JL--7|LJ|L-------JF-JF-7.
LJ7LJF7JJ..L--JLJL7L----J|F7FJF-----J|F-7||F77|L7|F--JFJL7F--JFJJ|L-J|LJL7FJ||L-7F-J7||FJ|||F-JFJLJF7L--JLJL-------7F7FJL-7L-7F--7F7FJL|77L7
.J77.LJF|FF7F7||F-JF-7F-7||LJFJF----7||FJ|LJ|FJFJ|L--7L-7|L7F7|F-JF-7L7F-JL7||F-JL--7|||FJ|||F7|F7FJL--------7F-7F7LJLJF7FJ|FJL-7|||L--7FF7J
||LF-77|--|LJL7FJF7|JLJLLJL-7L7|F7F7LJ|||L--JL7L7L7F7L7FJL7|||||F-JJL-JL-7FJLJ|F--7FJLJLJJLJ||||||L-7F-----7LLJ-LJL---7||L-7|F7FJ|||F--JJL7.
LL-7J-7|FJL--7|L-J||F7F7F-7FL-J||||L--JL-----7|FJ-LJL7|L-7||||||L--7F----JL7F-J|-FJL-------7|||||L7FJ|LF--7|F7F7F-7LF7LJL7FJLJLJ|LJ|L77||F77
LF-J|||7|JF--JL---J|||||L7L-7F7LJLJF7F7F7F--7|LJF7.F7||F-J||||||F--J|F7F-7FJL7FJFJF-7F-7F7FJ||||L7LJFJFJF7LJ|||LJFJFJL7F7LJF7F----7L-J--7L|J
|LJFJ7LL7.|F7F-7F-7|||||JL-7LJL7F7FJLJLJLJ-FJL--JL-J|LJ|F7||||||L--7LJLJFJL7|LJJL7L7||FJ||L7LJLJ-L7FJJL-JL--JLJF7L-JF7LJL--JLJF7F7||L|.JJ7.F
LL-7J7-FF-LJLJ-LJFLJ||||F7LL-7FJ||L--7F---7L-7F-7F7FJF-J|LJ||LJ|F--JF---JF7|F----JFJ||L7|L-JJF7F--J|F-----7F7F7|L--7|L--7F--7FJLJ|L7LJ.|FJ7J
|.F.FL7J||F-----7F-7|||LJ|F7FJL-JL---J|F--JF7|L7LJLJFJF7|F7|L77|L--7L---7|LJL7F7F7L7LJFJL7F-7||L---JL----7LJ|||L--7||-F7LJF7|L--7|FJ-L7--F-7
LFL-J.|FLFJF7F-7|L7LJ|L-7|||L---------JL-7.||L-JF-7FJFJ|||LJFJFJF7FJF---J||F-J|||L7L-7L--JL7LJ|F---------JF7LJ|F--J||FJ|F7||L--7|LJL7L77FLJL
LLJF|7L-FL-J||FJL7L-7L7FJLJL----------7F7L-JL---JFJL-JFJ||F7|LL7|||FJF7F7L7|F7|LJ7|F-JF7F7-L-7|L------7F--JL-7||F-7|LJFJ||||F-7LJJJJ|7||LL77
|.FJ.77L7F|.LJL7FJF7L7|L7F7F7F--7F---7LJ|F7F7F---JF7F7L-JLJ|L-7|||||FJLJL7|LJLJF--JL--JLJ|F--JL-------J||F---JLJ|FJ|F7L-JLJLJFJF7LL-7.F-.|-|
|-7-|.FFF--7F7FJL7|L-J|F||||LJF7LJ7F7L-7||||LJF7F7|LJL-7F7FJF7|||||LJLF--J|.F-7L7F7F7F-7FJL-7F7F7F7F7F-JFJF7F7.FJL7LJL--7F--7L-J|77.|.----.|
.|FF7-FLL-7|||L-7|L--7L-J|LJF-J|F--JL--J|||L--JLJLJF-7FJ|LJFJ||||LJFF-JF-7L7L7L-J|||LJ|LJ7F7LJLJLJLJLJF7L-JLJL7L7FJF7FF7LJF7L---J--JF||F.LFF
F7..--F-F-J|||F7||.F-JF-7L7|L-7|L--7F--7LJL---7F-7FJ|LJFJF7L7LJ||F--JF7L7|FJ7L--7||L7-F--7|L7FF-----7FJL--7F-7L-JL-JL-JL7L|L---7|-L.LJLJ--7.
--|7JLF-JF7LJLJLJL-JF7|F|FJF7J||F7JLJF7L77F--7|L7LJ-F7LL7||FJF7LJL7F7||FJ||FF---J|L7L-JF-JL7L7L----7|L---7|L7L-7F-----7FJFJF---J7J..FF7LJ7J|
LL||LLL7FJ|F---7F7F7|||FJ|FJL-JLJL7F-JL7L7L7FJL-JLF7|L-7|||L-J|F7LLJ||||FJL7L7F-7L7L7F-JF7-L7|LF7F7||F---JL-JF7|L----7||.L7L---77.|.7|LL.J.|
FJ|7J.LLJFJ|F--J|LJ||||L7|L------7|L-7FJFJ.|L----7|LJF-J|||F--J||.F-J|||L7FJJLJJL7|FJ|F-JL--JL-JLJLJ|L------7||L-----J|L--JF7F-JF7LF.LJ|.|.J
F-7-.--LLL-JL7F7|F-J||L7LJJF7F7F7||F7|L7|F7|F----J|F-JF7LJ|L---JL7L-7|||FJ|-FF---J|L7||F-7F--7F----7||F----7LJL--7F--7|F7F-JLJF7||F-7JF|77.|
|-JF7J.|LLLLFJ|||L--JL-JF7FJLJLJLJLJLJFJLJ||L-7F7FJL--JL7|L7F--7FJF-J|||L7|-FJF7F7L7LJ|||LJF-J|F---JL7L--7.L--7F7LJF-JLJ|L----JLJ||FJ.-7|7.F
F7|7..F77.||L-JLJF----7FJ|L---7F7F-7F7L--7LJF-J|LJF7F7F-JF7LJ-FJ|LL-7|LJLLJ|L-J||L-JF7LJF7JL--JL----7|-F7L---7|||F7L---7L--7F-7F7LJL7-F|--FJ
|||L-|J.|FF-JF-7JL---7|L7|F7F-J|LJFLJL--7|F7L--JF-J||LJLFJL-77L-J7.LLJ|L7FF----J|F--J|F7|L-7F--7F--7|L7|L--7FJ|||||F---JF-7LJFLJL---J--||F7|
.L---LL-.LJJ-L7L7F7F7||FJLJLJF7|F7F-----JLJL----JF7LJF7FJF--J7J-|||L|F-J.LL-7F7FJL7F7LJLJF-JL-7|L-7|L7LJF-7||FJ||||L---7|FJF7F7F--7F|FL|-7L7
|JFL77.L7L|J|LL7LJLJLJ|L-7F--JLJ||L---------7FF7|||F7|||FJ7F7F--7-|L|J....||LJ||F7LJL---7|F7F7|||FJL-JF7|FJLJ|FJLJL-7F-J||||LJLJF7L777J|L|L|
7L|J.LF-F7LFFJLL-----7|F-J|F7F--J|F--7F-7F-7L-JL7|LJ||LJL7FJLJF-J7LFFJL-7.LJ.FJ||L------JLJLJLJL-JF7F7||||JF7LJF---7||FFJL7|F-7FJL-JJJ-J7|FL
LF|-JFJ7L|7F--7F-----J|L--J||L--7|L7JLJ7|L7L7F7FJ|F7||F--J|F--J-F77|.77F777.FJFJL----7F7F---7F7F7FJLJ||LJL-JL7-|F7FJ|L-JF7LJL7||F---7JJL|.|J
--.-7||LFF7L-7|L7F---7L7F7FJL7F-JL7L---7L7L7|||L-J|LJ|L-7||L----JL777.L7.FJ7|FJLF----J|||F7FJ|||||F--J|F-----JFJ|LJJ|F--J|F7FJLJL7F-JL-JL--7
L-7.||LF-JL--JL7LJF-7L7|||L7FJL--7L----JLL7|LJL---JF7|F7L-JF7F7F7FJJF-.|-L|FLJ--L7F7F7||||LJFJLJLJL---JL----7FJFJF7FJL--7LJLJLF-7||F77|F7LL7
LFF||F7L--7F--7|F7L7|J|LJL-JL7F7.L-----7F7LJF7F--77|LJ||F7FJ||LJLJ||F.F--LL-|J.|JLJLJ||||L7FJF7F7F7FF7F7F---J|FJFJ||F7F7L----7|FJ|LJ|7-|J.LL
F7J.---.|.LJF-JLJL-J|FJF----7|||F----7FJ|L-7|||F-JFJF7|LJLJFJL--7|FJ7-JF77|FJJ7F7LF7FLJ||LLJFJ|||||FJ|||L----JL-JFJLJLJ|F----J|L-JF7|-FJFF-|
FJ-777LF.7J||F7F---7|L7|F7F-JLJ|L---7LJL|F-J|LJL--JFJ|L--7JL-7F-J-L7J7F7L--7|L-F--JL---J|F7FL7LJLJ|L7LJL--7F7F-7FJF7JF7LJF-7F7|F-7|LJ7L7FLJ|
-JF-LJFF77.-LJLJF--JL-J||LJF7F7L----J-F7||LFJF7F-7FJLL7F7L-7FJL7J.LL.FJ|7..|-7FL-------7||L--JF7F7L-JF---7LJLJ7|L-JL-JL-7|FJ||||7|L7L|JFFL7J
L|.7JJL||F7|F7F-JF-7F7FJL-7|LJL7F7F---JLJL-JFJ|L7|L--7||L7FJ|F-JJF|L.F.LFJ7JLL7J|LF----J|L----JLJL7F-JFF7L---7FJF7F-----J|L-JLJL7|FJJJFFL-|7
||7LJ|L|LJL7||L--JFJ|||F--J|F--J|LJF---7F--7L7|FJ|F--JLJ-LJLLJF7|FJ7F-|JF7|7.||F7JL7F7F-JF-----7F7LJJF7|L--7FJL-J|L7F7F7|L7F7F7FJLJ7LF|.L.L7
FJF|.|FL--7LJL-7F7L-JLJL--7||F-7L7FJ7F7LJF7L-JLJFJL------7JLF-J|7-JFLJ|.|LJ77F|F|-FLJLJF-JF-7F7LJL---JLJF-7|L7.F7L-J||||F-J||||L-7F----7J.||
.LFJF-JLFL|F7F7LJL--------J||L7|FJL--JL-7|L-7F7.L-7F7F---J77L-7L7|JF.F|--7L|JFL7JF----7L--JFJ|L7F-------JFJL-JFJL-7-|LJLJF-JLJL--JL---7|.LJ7
|-J-JJJ.77LJLJL-----------7||FJLJF7F7F--J|F-J|L--7LJ|L----7FF7|FJF---7J-LLJ||LJ.-L---7|F7F7L7|JLJF---7F7FJF7F7|F--JFJF--7L--7F----7F7FJ|7|L|
L7JL7J-F|J.F--------------J||L7F-JLJ||F-7||F-JF--JF7|F-7F-JFJ||L7|F--J-77J.L|.FFFF--7||||||.LJF7FJF-7|||L-J||||L7F7|FJF7L---J|F-7FJ||L7|F7--
.|JLJ.L7J-FL----7F-7F7F7F-7|L-JL---7|LJFJ|||F-J-F7||LJLLJF-JFJL7|||F-77F7.F7JF-7-L-7|||||||F--J|L-JFJLJL---J|||FJ|||L7||F----JL7|||||||LJL7J
|-FF.|.J.-.LLF-7LJFLJ||LJJLJ|F-7F--JL-7L-JLJL7FFJLJL---7JL-7L7FJ||LJFJFJ|7-|F|FJF7FJLJLJ|||L--7|F--JF-7F----JLJL-JLJFJ|LJF-----JLJFJ|FJF--J-
.FF-.|-F-J.F7L7||F7F7||F--7F7L7|L7F--7L--7F-7L7|F--7F--JF7FJFJL7||F-JFJFJJ.FFJ|FJLJF---7LJL7F-JLJF-7|FJ|7F7F7F7F7F-7L-JF7L--77F-7-|FJL7|F7|J
77|7FL7FJ77||FJ|FJLJ|LJ|F-J||FJ|7LJF-JF-7|||L7LJL-7|L--7||L7|F-J||L-7L7L7-F7L7|L---JF-7L--7|L-7F7L7|||FJFJ||||LJ|L7L7F7||F-7L7|FJFJL--JLJL77
LF-7-L||-LFJ|L7||F-7L--JL--J|L7|F--JF7|FJLJF7L----J|F-7LJ|FJ|L-7||F-JFJFJ-|L7||FF7F-JFJF--J|F-J|L-JLJ||-L7LJ|L-7L-JFJ||||L7L7LJL-JF-7F7F7FJ7
F7-7J||77.L7L-JLJ|FJF7F7F7F7L-J|L---JLJL7F7|L--7F-7|L7L--JL7L7FJLJL-7L7|F7L7|||FJ|L-7|JL--7|L--JF--7FJ|F7L-7L--JF77L7|LJL-JFJF--7FJFJ|||||F7
L|7|-|||LF7L7F7F7|L7|LJLJLJL--7|F7LF7F-7LJLJF--J|FJL-JFF-7|L7||F---7L7|||L7|||LJFJ7FJ|F---JL---7L-7|L-J|L-7L--7FJ|F7|L----7L7L7FJL7L-JLJLJ-|
.-7..|F|L|J7||LJLJFJ|F--------JLJL-JLJFL7F--JF--JL----7|FJF7|||L-77L-J|||FJ||L-7L7FJFJL--S---7FJF7||F--JF7L--7LJFJ||L7F---JLL7||F7L7|F---77J
F7F7FFJ|||7FJ|F--7L-JL--7F-----7F------7|L---JF-7F----J||FJLJLJF-JF7F-J|||7||F7L7||FJF-----7|||FJLJ|L---JL--7L-7L-JL7|L-----7||||L7L-JF-7L-7
|FJJJJ.L7.LL7|L-7L------J|F----J|F-----J|F7F-7L7||F7F-7|||F--7FJ|FJ|L-7||L7||||FJ|||J|F7F-7|FJ|L--7L7F------J.FJF7F-J|F---7FJ|||L7|F7FJJL-7|
LF|J7F7.|-L|||F7L---7F7F7||F7F-7|L--7F-7||LJF|FJ|LJ|L7|||LJF-J|F7L7|F7|LJFJ||||L7LJL7LJLJF||L7L-7L|FJL-------7L7||L-7|L7F7LJJ||L-JLJ|||.FLLJ
LJJFJLJJ-F.FLJ||F7F-J|||||||||FJ|F--J|FJLJF-7LJFJF7L7||||F7L-7LJL7|||||F-JFJ||L7L7F7L7FF7FJL-JF7L-J|F7F---7F7|FJ||F7|L7LJL7|-LJLF---JL-77J||
||LJ.FJLL7FF7L|LJLJF7|LJLJLJLJL7|L---JL7F7L7|F7L-JL-J|||LJ|F-JF7FJ|||LJL--JFJL7L-J|L-JFJLJF7F-JL--7LJLJF-7LJLJL7|LJ||FJF-7L7F-7.|F--7F-J-.77
FL.L||.|FF-|L7L7F7FJ||F--7F-7F7LJF7F--7LJL-J|||F7F7F-J|L-7|L7FJ|||||L7F----JF7L-7FJ-F7L-7FJ||LF---JF7F7|FJF7F-7|L7FLJL-J.L-J|FJ7LJF-JL7JL7LJ
LL-7J7FF7||L7L-J||L7||L-7|L7LJL--JLJF7L-7F-7||LJ||||F-JF7||FJ|.LJFJL-JL---7FJL--JL7FJL--JL7LJFJF7F7|||LJL-JLJFJL7L---------7||-F|J|F-7|.FFJ.
|JJF-J7LLL|.L---J|FJLJF-JL7L-----7F-JL-7LJJLJ|F7LJ|||F-JLJ||FJF7JL-7F-----JL7F-7F-JL------JF-JFJ||||||F------JF7|F7F-7F----J||.F7.LJFLJ7||L7
|J.JFF7.LJ|7FF---JL7F-JF7FJF7FF7JLJF---JF7LF7LJL-7|||L---7||L7||F-7||F-----7LJFJL7F-7FF7JF7L-7L7||LJ||L7F-----J|||LJJ|L-7F7F||FJL-7FL-|.L7-|
|J.FL7.7FF-7-L-7F--JL--J||7||FJL-7FJ7F--JL7|L7F--JLJL7F--J||FJ||L7LJLJF----JF-JF-J|FJFJL-J|F7|FJ||F-JL7LJF-----J|L--7L-7LJL-JLJF--JFJJF7|L7|
FF-|JL7LLJLJ-|-LJJF7F7F-JL-J|L--7|L--JF7F7LJFJL---7F-JL--7LJL7||7L--7FJ|F7F7L-7|F7|L7L-7F7LJ|LJF||L---JF7L-----7|F-7L7FJF7F---7L---7J7J|7.F|
LL-F7.-7L7---LF---JLJLJF7F-7L---JL----JLJL--JF-7F-J|F7FF7|F--J|L7F-7|L--J||L7FJLJLJFJF7LJL-7|F-7LJF7FF-J|F7F-7FJLJ7L-JL7||L--7|F---JJLF|FJJL
F|.|LL7|.L7JJFL---7F7F-JLJJL-------7F7F7|F-7-|FJL-7|||FJ||L7F7|FJL7LJF---JL7LJF-7F-JFJL7F7FJ|L7L7FJL7|F7LJ|L7LJF7-F7F-7||L7F-JLJJJF7J.7-L7FJ
F--|77-|.|..FF7JF7LJLJJF--------7F-J|||L7L7|FJL7F-J|||L7||FJ|||L7FJF7|7F77FJF-J|LJ-FJF-J||L7L7|FJ|F-JLJL-7L-JLFJL7|||FJLJFLJF-----J|---J|L|7
|L---FJ|-7.FFJL-JL-----JF-7F---7|L--J||FJFJLJF-JL-7||L7|LJL7|||FJL-J||FJL7|FJF-7|F7L7L7F||-L7||L7||F-7|F7L---7L7FJ|||L-77F7F|F-7F--JF|JFJ-|7
FJF|-F7F7|FL|F----7F7F-7|FJ|F--J|F7F-J|L7L-7FJF7F7|||FJL--7|||||F77FJLJF-J||-|FJFJL7|FJFJL7FJ||FJ||L7L-JL----JFJ|7|LJF-JFJL-JL7||.LLF7F|J.LJ
|F-L7|JL7-|7LJF---J|LJFJ|L7|L7F7LJLJF-JFJLFJL7|||LJ|||F7F-J||||LJ|FJF--JF-JL7|L7|F7||L7L7FJL7|||FJL7L----7F7F7L7L-JF-JF-JF----JLJ.7|||L.|7||
FL7.J7.7L-J-LFJF-7FJF-JFJFJ|FJ|L7F-7L7FJF7L-7||||F-J||||L-7||||F-JL7|FF7L-7FJ|FJLJ|||FJ|||F7|||||F-J|F7F7LJLJL7L7F-JF7|F-JF----7LF7JFFJ-L|J7
|F--F|-J7-|7LL-JFJ|.L-7|FJFJL7|FJL7L7|L7||F7||||||F7|||L7FJ|||||F-7|L7|L7FJ|FJL7F7|||L-7||||||||||F-7|LJ|F----JFJL-7|||L--JF--7L--7J-J|L||LL
FJ.L7J|LJFF7.L.FL7L7LFJ|L7|F7LJL7FJFJ|||||||||||||||||L7||7LJ||LJFJL7LJFJL7||F7|||||L7FJ|||LJLJ||LJFJL7FJL-----JF-7LJLJF7F7L-7|F--JJ-FF.FF7.
L--J|L77.7JL.LLF-JFJ-L-J.LJ|L---JL7L7|FJ|LJ||||LJLJ|||FJ|L-7.|L-7L--JLFJF-J|LJ|LJ||L7||FJ||F---JL-7L7FJ|F7F7F-7FJFJF---J|||F-JLJF7J|JFF7|||-
FFFFF-L.FJLLL7LL-7|JJLF----JF--7F7|FJ|L7|F-J|LJF---J|||FJF7L7L-7|F----JFJF-JF-JF7||FLJ|L7|||F7F--7L7||FJ||||L7||LL7L---7LJ|L-7FFJ|LL---J-J-7
F--J|FJ7|-F7.F7||LJ7F|L-7F-7|F-J|LJL7|FJ||F7L-7L-7F7|||L7|L7L-7|||F---7|.|F7|JFJLJ|F-7L7|LJ||||F-JFJLJL7||||FJ|L-7|F7F7L-7L-7L-JFJ--J||LF|.F
L7|FJ7L-JJL7.|L|F|FJ-FF7||-LJL7FJFF7LJL7|LJL7FJF-J|||||L||FJF7|LJLJF--JL7LJ|L7L--7LJFJ.LJF-J|LJL-7|.F7F||||||FJF7|||||L7FJF7L7F7|JF7JF|LFJ-L
LL7LF-7|LFF|-7FL-L-.F-JLJL7F7|LJF-JL---JL--7||JL-7|||LJFJ||FJ||.F-7L--7FJF-JFJF--JF7L---7|F7|F-7FJL-JL7LJ||LJ|FJ||||LJFJL7||FLJLJJFJFL7-L7.|
F-JLF-FJ-FL|.7-|.7|FL-----J|L---JF7F7F7F7F7||L7F-J|LJF7L7||L7|L7L7|F7FJL7L7FJ.L--7|L-7F7|||||L7|L7F--7L7-LJF7||FJ||L7FJF7LJ|F7F7JF|.-JJ7.|F-
F7|L|||FF77|7|||-|-FF7F----JF-7F7|||||||LJLJ|FJL7FJ.FJL-J|L-J|FJJ|||LJF7L7|L7-F--JL7FJ|||||LJFJL-JL-7L7L---J|||L7|L-J|FJ|F7|||||.F7-L777FLJ|
|-.LL|7F||7L-L-7-F--J|L---7FJFJ|||||||LJ7F--JL-7|L7FJF7F7L--7LJF-J|L7FJ|FJL7L7L7F7FJL7||||L77L--7F--JL|F7F7FJ||-|L-7FJL7|||LJLJL7||-|LJ77|||
-7|FL.-JJ77J.FFF.L--7L----J|LL-JLJ||||F--JF-7F-JL-JL7|LJL7F-JF7L-7L-JL7||F-JFJFJ||L-7LJ|||FJF---JL---7LJLJ|L7|L7|F-JL7FJLJL7F7F7LJL-7-L|JF77
FJ7-L|FLFLJFF7-|FL-L|F7F---JF-----J|||L--7|J|L--7|F7|L7F7||F7||F-JF7F-JLJL-7|LL-J|F7L7FJ|||-|F---7F--JF--7L-JL7||L-7FJ|F7FFJ|||L-7F-JF-L--JL
LLL7|||.F-LL7.LL7L77||LJF7F-JF7F7F-J|L7F-JL7L7F7L-J||FJ|LJLJ||LJF7||L--7F--JL--7J|||FJL7|||FJ|F--J|LF-JF7L---7|||F7|L7LJL7L7||L7J|L-7J-FJJL|
.FJ|77L-J7||JL7F7-7-LJ|FJLJF-JLJ||JFJFJL-7FJ-|||F7FJ||7L--7FJL-7|||L7F-J|F-7F--JFJ||L-7|||||FJL7F7L-JF-JL7F7FJ||||LJFJF-7L7||L7L7L--J||FJJ.F
L|L|L||.LLF-7J|F.L77.LL|F7FJF---JL7L7|F--J|7FJ|||LJFJ|F---JL7F-J|LJFJL-7LJFJL-7FJFJ|F7|LJLJ|L7.LJ|F--J-F7LJLJLLJ|L7-L7|.L7||L7|FJ|7.F-7.F-77
L|.-.LF-.|.FJ7F|..||J..||||-L7F-7FJFJ|L-7FJFJFJ|L7-L-JL---7FJL-7|F-JF--JF-JF7FJL7L7||||F-7FJFJF--JL---7|L---7F7|L7L7FJL-7LJ|FJ|L77JJ-L77F|L7
LF-J77LJ-L-FJL7..F|J..-||LJF-J|FJL7|FJF-J|.L7|LL7L--7F7F--JL--7||L-7|F-7L-7|||F-JFJ||LJL7LJFJFJF-7F7F7LJF7F7LJL-7|FJ|F7FJ7.LJ-L-J|.|L|LFF|FJ
-L-.|-7F.|.|.LL7F..LF-L|L7|L-7|L7FJ||7L7FJF-JL-7L7F7LJ|L--7F--JLJJFJLJFJF-J|LJL7FJ7LJF--JF7||L7L7LJLJ|F7|LJL----J|L7LJ||JJ-LLJLL-|7LFF-7L|J7
|LF7|LFJ|L7.L7.L7-L---.L-J-F-JL7|L7LJF-JL7L-7F-JFJ||F7L7F-JL7JF7FFJF7FJF|F7L7F||L---7L--7||L7FJFJ7F7FJ||L7F7F7F7FL-JF7LJJL|7L-|J|LJ7L7-F-JF|
F7|JF-L-7JJ.FJ7.|FL|JFJ.LJ||F7FJ|FJF-JF-7|F-JL-7|FJ|||FJ|F-7|FJL-JFJ|L-7||L7L7FJF7F7|F--J||FJL7|F-JLJFJL7||||LJL----JL7J7LJJJF|FFF-77|7|7F-7
L-|-|.L7.|.F|-77L-J|7|J.|-F|||L7||J|F7L7LJL-7F-J|L7||||FJ|J|||F-7FJFJF-J|L7L7|L7|LJLJL7F7||L7FJ|L7F-7||FJLJLJF-----7F7|JFF7.--J77|-JF-F----|
F-77|7||-.LLF7FJJ7||-J..7.LLJL-J||FJ||FJF---JL-7|FJ||LJL7L7LJLJFJL7L7L-7L-JFJ|FJL-7F--J||||FJ|FJJLJFJL7L-7F-7|F7JF7||LJ.LFL-|7L--J|..F||L|||
L-L-F7F|7|F|LJLL--F7-7.7|-LFLLJ|LJ|FJLJ|L-7F7F-J||F||F--JFJ-F7-|F-JL|F7L--7L7|L7F-JL7F7|||||FJ||F7FJF7L-7|L7|LJL-J|||.L|-F.|LJ7.FF-|7JJF7F7|
|-F7.FJJLJJJ|.JFL7.|F|J.F-77.|.FJ-LJJFLF--J||L-7|L7LJL7F7|F-J|FJL--7||L7F7|LLJL|L7F-J|LJLJ||L7|FJLJFJ|F-JL-JL-7F--JLJ.L|LJ-|7.J-L-..||F-JJFJ
7-LLFLJ7F--F7..F.L-|LJ.F-7|LL--J-FFJF|-L--7||F7|L7L7|L||LJL-7LJF7F7||L7||||F---JFJ|F7L-7F-J|FJ|L7F7|F||F----7||L-77|JL7L-..-.|7LF|7.|7FL.F||
|..FJ7F-J|F|77LF-JF|7LFLJ|-FJFF7FF7-F-.F--J|LJ|L7L7|--LJ|F7LL-7|LJ|||FJLJ||L--7FJJ|||F-JL-7||FJF||LJFJLJF7F7L7L7FJ-|7LL|-77J-FL-LLJ7F7LFL7L-
--FL--JJ-JJ.77.JJ|LJJ.|JJJ.JJFFJ-|L-F|.L---J|LL7|7LJ7.|LFJL---JL-7|||L-7-LJF--JL7FLJ||F---J|LJF-JL7FJF-7|||L7L7||7|LF.|L7L|..7-7-|F|FJ-JJJ.|
.F|.FJJ.||F7.J.L-J.JF7J|LJ.L.F|JFJ7LFJF7|-L-|J.LJJJ.|7|-L----7F--J|||F7L7.LL7F7FJF--J||F--7L7-L7F-JL7||||LJ.L-J||--JLF-7J7L7..F||L-||L-J.L7J
F7F|-77-JFLJF|F7LLFLF-F77J|-JLJ-JJ7.F-||J7J.L--J7|-J||7F-7F--JL--7|||||FJ7-|LJ|L7|F7FJLJF-JFJF-J|F--J|FJL----7-LJ7.F-FL|LJ7JF-F|JJ.L77FFF-J|
7L7LLLF-L-FF-|LF..7FL|J7|-LF.F-F.L|F7.FL-L-F-F7.FL7FF|-L7|L7F7F7FJ||||LJJ|-F7FL7|LJ|||FFJF-JFJF7|L-7FJ|F7F7F7L7F7-J-..|LJ||.|-FJJLFLJFJ7|--7
L-|L||LJ|.LJJF-7.--7F--7J---7|FJ|--.F7JL7|.|JL7FL.F-JF--JL-J|LJLJFLJ||-LLL-L7JFLJ..||77L7|J.L7|||F-J|FJ|||LJL7|||.|F7-77.-|7L-L7FF-.F|LF--LL
7JJLF|7-77L|L|-7F|-F-.JFJJ--LF7-|-LFLJJF|-FL-LJJ7FJLF|F--7F7L--7JJJJ|||F.F7-LJ7-F|7LJ|FLLJJ.FJ|||L7FJ|FJ|L--7|LJL777|FF7J.LJLJ||FL|7F77L||F7
J7F.FFJ-L--L-L7||J.7|FJJFJ.7JLL..L7.|JLL|L-JF||-F|7FFLJF-J||F7FJJF77LJJ|FJ7..-7-FJFJF77J7.LFL7|LJ|LJ-LJ||F--JL--7L7.F|JLL|JF-7|FJFL7-|-7F-JJ
J7|F|J-FJL-L7LL|J.J.|7.|F-7JFL|-7F--JF7.L7FL7F|JF-J-JF-JF7|||||LF-|-LJL|-FFJ7.|.LF|-LJ|LF7|FFJL77F------JL-----7L7|7|L7F--77LLFL-7.||LFJJ|7J
|J-F|JJJJ|JLJ.LJ-7JF7-L-7LL7JLLJL7JLLJJ|L|--LL--|-J.FL--JLJLJLJ-|LF7J|.|.F7L--LL-|J7.LJL--FJL--J-L-------------JLLJ-JJ|.LLL-7--JJJ--7-FJ-LJ-";