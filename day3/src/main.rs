use std::collections::BTreeMap;

fn main() {
    let input = r"
........936..672.........846.922........359...332......582..856........................579..93......674..740.....243.156....................
...........%.........4=...*...*........*.......*......#....................806..481.........................*.......*.........900......$564.
.............520........624.965....143..405.....960.............273...651...*....*.........554....139@.....38...*.........58..*...392.......
................$....................*.........................*.....&......634.3..../.................-......310....*313.*.........*.......
...196....544......541................775.216...+557..225/..463.......................517..........960.228........452.....593.......772.....
.....*....*...654.*........638............*...........................163....................386#......................20......169..........
.....566..20...$...132........*969......747........*971..342............=.245*.......................277.522............*..............+....
.............+..............................205.716...........................855.............324......................871..752.........291.
.........47..390.......128...%....528........%..........91*192.......=.....................=.#.............%..852*156......+................
.....975..*.......=....*...322.......#............&63.............946...................462....939..520...268............#.....*....380.....
.........55..=.....871..68.............................................491.......................*.....*............674.770.333.415...*.275.
..............633...........@.....642.5........+.......739........632.....*.......992.74.615...858...998.............*..............761.*...
....446......................180./..../........83...&.....*924....&........379...*......................................................118.
...........488*240.......150......................958..........$.....&...........964...351...........+..633.........123....../603...........
...................987....../........546-...674...............466.....602.............@...........956..#............./......................
......503=....465.................91.........*..........................................583..............128..124..........$.....217........
..............*...32$..53....&594.........648...827....*..994.......971...................*..........304...%....*.......387..80.............
............756.......#..........................*..888...$............@....339.......+..868........*........851..............*.........128.
...905-.........312..................729...356*.697.........736.....+.......+.......748.......797....41....................272..753.........
.................*....&604............*.............%915.......*.529..889...................*.........................992.........*.....718.
...........272....487..............+...961.892..............632........*.................502.118....141................@......546.669.......
787..........*....................85............/..330...........-...374.............................*.........446...%.........*............
....-...../...120....919&.....................339....*....*663...403........167...533.170.............698..587...*....279.......776.+.......
.575.....34.*.................668...19..759.......804..730...../.............*...........*.....789...........*....803......666.......25.....
............375...773*.............*.........................$.876...287......201........484......*.........196.............................
565...866.............23..477...455....................794.996.........*.....................887.959...146...........496....790.............
..............648...........+........325.738....156.....*.........534..941.....%536..31..../.%...........*............*.....*.......511.....
..............*.................166.....*......*.....647..........*......................262...........887..210.3..860.....161....4....*420.
......141.606...........193.111..*..........252......................935*689..................500...........*...*.................*.........
..635....*........=........*.....959.....................@834.................-.....542..547....-..........770.8...........439..............
.................896.........93......................244..........@......247..735..*.................................497.............%......
...............................%........................*..........200....+.......342.613.245.@............$.........#....*.......176...21..
......51......702.37.......798....325.....&974.....834.860.....+.........................*.....533..866....864.............269..............
..141..*......................-...&............875*.............207...882*.........................*............309.739........%......../...
.......394....144.....................................$...................835.3..........290.......489...........&.....*177.833....866$.153.
.689.................719.314/..$......673.....5*557...443......+..#649.............575....*..634*...........................................
....&...........794...*........614...*......................350.............411......*.104.......141........369...934...........997.........
.......519......*...787..............739.....628...................................784.......$..........989*........*...987.......*....#....
.........*...309.........209.............957*........50.....@995...765.......269.@........380..624.................643.....*....836...356...
......886............234.+....................51*935..*...........*.....188.......73..............*696...14.....#...........................
............203..948...&...............................957......804.736*......388....689.219*660........*.....218....................$..40..
..............*.....*.....475.....930..........%................................*......*..............477.624................848...687......
....57*.50.229....636....*.................804.164....590.........-....547..80..310....675.......=.@...........................*............
........................398......&........*..............*.....748...........*................226..200................160......853..........
......292........................889.......867.....365...............192+..530.........................@910....=..222*.............894......
..427....#.../...959........................................322.....................735*304....742..........195..........$..............22..
...........465..-.........52.$......168.......418.......+.........................................$..............&......563...649*26........
....................464..../.969....*......-..*........150......724.....83..301...............527...............858.506.....................
...935.........910....+...........116....427...347.............-........*.....=.......891.......*....847...750........@................531..
....*.........$.........676...............................510............11.......647.*.................#.....%.683.............193...+.....
....761.169...............=...524........&......152..........*...975.994.........*....122..........858*...........%.......776...........*...
...........-..180/.850..............$.....524....-...........940.=......*......199........963..............#........836...*.....34...543.448
482...947...................94&....541..............265..............477..890..............*....988.....343..........&...599....+...........
.......*...532..81..73..417............992/...99&...#........................*.33@.149..305.........678.....31.........@.................417
....304.....@.....*...#../....452.842.................519.....307.949.....268................=.........*....*...........259.................
..................361............*.......346............*.+......*...........................288.......492..985.............727.683.........
....530....438..........950..........*......*........787..316...............762.724.257..889.....................844.......*......=....@125.
..............*...446...*......879..827......148......................*730.......@..........*...832...............*.......647.......48......
....500.......994........41.......*.....#805...........*...........614...............*....940................291&.129.8............=........
....*...#..........698..........581.................940.462.....@......443...82.784.593......../.#375..%148............*..#.................
.....39.562...784............................................805...$......@...*..*..........844.....................776....363...667........
................%...265.......337.....438...295.....................499......341.74..................271.405.......................%.155....
.........97..........*......@....*252.*......*....+.........270.229.................394...879*247.....*...*.....146.....................*...
........%.............226..258........996.944......793........*.../...................+..............321.578.....$..........838.....529.831.
.................%109.........................................615....822.....................501.............616....293.....&........-......
.......%....482...........................%717.236.......562...........*...........915...580....*............*.....+..........230...........
.....303.......*128..86&......762.928.............$........*......861*.311........*....../....517..........471.544.....996.......*38...628..
...........563............89.*......*.583...%.148.......$.987...................35..................996...........=....*...............*....
......776.....+...........*......967...@..168...*.....772.........838*.....480..............................-........976..............530...
................649*32...84.#...................315...................37............398.656...631..........340.985*..........522............
............................922........................./.173....894................*.........$....................177.......#..............
......................................................423....*....*.....816.........13.................=..............................251...
..154...................878*......................568......723..329....&......928.................-71.123.378..100...................*......
........*21..250............448.734.718.....&....*......................................599...............%....*.....700+.....*803..401.....
.779.550........*.817............./..*....118....980..589.....235......=....539...........*...281*236..........591.........927..........*...
...-.................*...............854.............*.................911.=............939........................616..*............770.50.
.............82......399.......452..................184........643..........................*871..................*....370..................
.185.405.....*...634............*......189.838.................*...............@623......252...........315.86.....568...........157.104.....
...&......586.......*........957..........*.......841.........509.600........................896..565.....*...............+............*....
..................633.628...........811..........*.....677.........-..580..751...183.........................540.81.......656..665..634.....
......112.............*.....=........./.322...430.........*..........+.....*...............683............$.....*...........................
830&...*..426........649..910...........*................717..359........831........=..157..%.............143......758................579...
.....373....*..................215.......666....................*.#...........459..547...+.........773.........980*.........404...126.......
205......636....................*...420.......440..........225.11.516...568....*......................*....474........620...*.....*.........
..............250......685...209.....+...........*257.................../...407.......295..498....122.............970...@.392.412...........
783..83+...........706....=.................330*............833.................*250....%..=..........%...........*............#....898.....
...........$....................................398..35........*......646....759.....#.............527..........160.....254.................
500........777...........112..428...663....957#..............524.....*..../........249..530*672............567......*......*759.....284*....
....861....................*.*......*............*...277*832.....644.130..216.446............................*...683.955................950.
....*........826..........68.535..192.........306..................#..................762%...%....#631..500.406..........763...137..........
..65............-...499=..................546.....&.........$........../..........177........988.......@.............336...*..*.............
.....362..................@...$..........-....610..65......64.........830..........-...........................452......+.604.542...........
........*902.633*........873..63.....781...31...*.................258.....12...681..........18*........&...&............................848.
..803............752.........................-.....672...................*.............................470..86.59.......%...................
.......@..547..........$...184.......320..........+..........471......834...=...551..108.............................176.................82.
..950.562............261...$.....754..+..............134.654*...............778..&......%..713.837.......................329........186.....
............112..72...............*..........917......*.................415............................946....=....614.....*................
..............*.$......850.....766.....308.....*....666................*......812.....293................*.624.......%..296.....612...333...
..996......154...........................*......283.....*.555.%........................-.......552.714.563.......227..............-.........
....*.682.................=.......281.217..............84.*....908.....@...=....522..............$.*.......630......*79..............556....
..635...*............516...402............................333........114..496..*.......324..........522.....................................
.........511....779...*............@............350.853.........................791...................................$.....821.....+.......
.................*.....486........984......332.....*..............893..730*480...................756...................776...........155....
......161......57..590....................@....398..................*....................477.....*......602....506............996...........
.151.....*..=.......*......955.......916........#.....=..269.........423..........289......*..379...*..@.....................*..............
...-....123..927.600...........-........*395........226./....676*755...........30..........58.....582.........671.118..43.367...943......478
.....................725........74...............................................+...........................-......*..*................&...
........378......563*.....*999............*...................497.349...26.....$............362.........&........723....20....717...317.....
...*....................22.........../.575.552..........291......*..........600......152.../......./...169......................-.@....*977.
886.250..........$.............383.603.............36*.../....%......................*.............253........215..........139....583.......
................225............................................721...814..............829..................*.......613*544.*...=........901.
..........741.......595.........785.............%677..50................=.................252......710....91...............912.180..........
...........*...................*....808...956..........+.....812...............*381......*.....676.../..........599....................+....
............134.184......759..81...*.........*.782........41*......=....125.216........679...../..........@........*.................567....
.......165.........*......@................66...*.....*.........379.....................................&..626...241........................
.........*.........232.........387.86...........635.47.806.857.........$537...............240........617..............806..941..........531.
.......687...654.......735......*...$......................%.....357..............*884.......*571..........273.551......*.....*....576..*...
............*................648...................716.304..........*...690....663.................*38...........*....365......911....@..424
....67*156...250..828...................&222.......#...*......16....300.+..................-....-.9....813...982..443.......................
.....................*......136..757..................65..314...*........................615.291.......&....*................972............
................721..592...@.............................../...808.............665...176.............................200.....*..............
..198..459+............................218........-772..............169...39..*........*........939..742......#....=......532...#..264......
.....*...............$..........713...*............................*.....*...831....689...589*.......*.......91....75.........641...#.......
..896..167*........372...138...........462...*227..$.....801......512.........................775.673.......................................
...............47........*.......135...............725.....*...............743..........489*.............735..622.815.551...........519.....
.......=........*......636.987...*...................................827........496*........48.121................*.....*........#.....*311.
....525.......819.580.........#.369......119...............77.........*...............10*........*.%.....975.....961....888......304........
........................96*..........................816............857......=...........376...859.522..=...................................
629...419..259...#360.......44%.........190..=969...........=...539.....-913.429......................................119.....476...707.....
...*.....*...*....................*906.*.....................84.....286...............$...=...127.200.465........324.............=.....*599.
905.....762...262.....-597.....847......311......@654...837............*....455..37..439..147....*.....*...262...*.....787..778.............
...........................25...........................=...........908....*.......*................827......%....567....-....#.427..819....
..............................#.$....643=...............................886......243....../.....672..............................*....*.....
...........185*....107.....226..965........$.................756........................808.221...-.../574...541&...+....871...326....349...
......................*.....................841.....=...........*688........177................*....................166....*................
.......189..744......308.......99*391..630/........463......................@...930..........543..........................519...............
..984.%........-.741.......................................522.915+.....769......*................207....976.....158*.......................
....*...............*...............&.331...787........48...........224..*......184........874.......=.....*........................537.....
.....934....*339...829....495.....682...*.............*....+..........*..794..........-430...*....&........848..367....+............*....505
.........175..........................381............270....198......911...................52......642...............45............445......";

    let lines = parse(input);
    let mut index = 0;
    for line in &lines.lines {
        println!("{} - {:?}", index + 4, line);
        index += 1;
    }
    part_one(&lines);
    part_two(&lines);
}

#[derive(Default, Debug)]
struct Line {
    pub symbols: BTreeMap<usize, String>,
    pub parts: BTreeMap<usize, usize>,
}

#[derive(Default, Debug)]
struct Lines {
    pub lines: Vec<Line>
}

fn parse(input: &str) -> Lines{
    // split by lines, and check each previous/current/next line for being next to a symbol (anything but a .?)
    let mut total_lines = Lines::default();
    let mut lines = input.lines();
    for line in lines {
        let mut current_line = Line::default();
        let mut part = String::default();
        for (index, symbol) in line.chars().enumerate() {
            if symbol.is_digit(10) {
                part.push(symbol);
            } else {
                if part.len() > 0 {
                    current_line.parts.insert(index - part.len(), part.parse::<usize>().unwrap());
                    part = String::default();
                }
                if symbol != '.' {
                    current_line.symbols.insert(index, symbol.to_string());
                }
            }
        }
        if part.len() > 0 {
            current_line.parts.insert((line.len() - 1) - part.len(), part.parse::<usize>().unwrap());
            part = String::default();
        }
        total_lines.lines.push(current_line);
    }

    total_lines
}

fn part_one(lines: &Lines) {
    // check each line for a symbol, and then check the previous and next line for a symbol
    let mut total = 0;
    for (index, line) in lines.lines.iter().enumerate() {
        for (p_index, part) in &line.parts {
            let mut lines_to_process: Vec<&Line> = Vec::new();
            // check previous line
            if index > 0 {
                let previous_line = &lines.lines[index - 1];
                lines_to_process.push(previous_line);
            }

            lines_to_process.push(line);

            if index < lines.lines.len() - 1 {
                let next_line = &lines.lines[index + 1];
                lines_to_process.push(next_line);
            }

            let part_len = part.to_string().len();
            let mut is_match = false;
            for c_line in lines_to_process {
                let (result, _, _) = check_for_symbol(index, c_line, *part, *p_index, part_len);
                if result {
                    total += *part;
                    is_match = true;
                    break;
                }
            }
            if !is_match {
                println!("[{}] No Match: {}: {}", index + 4, p_index, part);
            }
        }
    }
    println!("Part One: {}", total);
}

fn part_two(lines: &Lines) {
    // find each * that is adjacent to another number and multiply them together, then add the result.
    let mut total = 0;
    let mut gear_symbols: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for (index, line) in lines.lines.iter().enumerate() {
        for (p_index, part) in &line.parts {
            let mut lines_to_process: BTreeMap<usize, &Line> = BTreeMap::new();
            // check previous line
            if index > 0 {
                let previous_line = &lines.lines[index - 1];
                lines_to_process.insert(index-1, previous_line);
            }

            lines_to_process.insert(index, line);

            if index < lines.lines.len() - 1 {
                let next_line = &lines.lines[index + 1];
                lines_to_process.insert(index + 1, next_line);
            }

            let part_len = part.to_string().len();
            for (n_index, c_line) in lines_to_process {
                let (result, symbol, r_index) = check_for_symbol(index, c_line, *part, *p_index, part_len);
                if result {
                    if symbol.as_str() == "*" {
                        if gear_symbols.contains_key(&(n_index, r_index)) {
                            let value = gear_symbols.get(&(n_index, r_index)).unwrap();
                            println!("[{}] {} * {} = {}", index + 4, value, *part, value * *part);
                            total += (value * *part);
                        } else {
                            gear_symbols.insert((n_index, r_index), *part);
                        }
                    }
                }
            }
        }
    }
    //println!("{:?}", gear_symbols);
    println!("Part Two: {}", total);
}

fn check_for_symbol(line_num: usize, line: &Line, part: usize, p_index: usize, part_len: usize) -> (bool, String, usize) {
    let mut sub = 1;
    if p_index == 0 {
        sub = 0;
    }

    for i in p_index-sub..p_index+part_len+1 {
        if line.symbols.contains_key(&i) {
            //println!("[{}] {}: {} = {} -> {}", line_num + 4, line.symbols.get(&i).unwrap(), i, p_index, part);
            return (true, line.symbols.get(&i).unwrap().to_string(), i);
        }
    }
    (false, String::default(), 0)
}