use {Value, Switch, Counted, Plain, List};
use test::{get_args, map_from_alist, same_args};

test_expect!(test_0_testcases, "Usage: prog", &[], vec!())

test_user_error!(test_1_testcases, "Usage: prog", &["--xxx"])

test_expect!(test_2_testcases, "Usage: prog [options]

Options: -a  All.", &[], vec!(("-a", Switch(false))))

test_expect!(test_3_testcases, "Usage: prog [options]

Options: -a  All.", &["-a"], vec!(("-a", Switch(true))))

test_user_error!(test_4_testcases, "Usage: prog [options]

Options: -a  All.", &["-x"])

test_expect!(test_5_testcases, "Usage: prog [options]

Options: --all  All.", &[], vec!(("--all", Switch(false))))

test_expect!(test_6_testcases, "Usage: prog [options]

Options: --all  All.", &["--all"], vec!(("--all", Switch(true))))

test_user_error!(test_7_testcases, "Usage: prog [options]

Options: --all  All.", &["--xxx"])

test_expect!(test_8_testcases, "Usage: prog [options]

Options: -v, --verbose  Verbose.", &["--verbose"], vec!(("--verbose", Switch(true))))

test_user_error!(test_9_testcases, "Usage: prog [options]

Options: -v, --verbose  Verbose.", &["--ver"])

test_expect!(test_10_testcases, "Usage: prog [options]

Options: -v, --verbose  Verbose.", &["-v"], vec!(("--verbose", Switch(true))))

test_expect!(test_11_testcases, "Usage: prog [options]

Options: -p PATH", &["-p", "home/"], vec!(("-p", Plain(Some("home/".to_string())))))

test_expect!(test_12_testcases, "Usage: prog [options]

Options: -p PATH", &["-phome/"], vec!(("-p", Plain(Some("home/".to_string())))))

test_user_error!(test_13_testcases, "Usage: prog [options]

Options: -p PATH", &["-p"])

test_expect!(test_14_testcases, "Usage: prog [options]

Options: --path <path>", &["--path", "home/"], vec!(("--path", Plain(Some("home/".to_string())))))

test_expect!(test_15_testcases, "Usage: prog [options]

Options: --path <path>", &["--path=home/"], vec!(("--path", Plain(Some("home/".to_string())))))

test_user_error!(test_16_testcases, "Usage: prog [options]

Options: --path <path>", &["--pa", "home/"])

test_user_error!(test_17_testcases, "Usage: prog [options]

Options: --path <path>", &["--pa=home/"])

test_user_error!(test_18_testcases, "Usage: prog [options]

Options: --path <path>", &["--path"])

test_expect!(test_19_testcases, "Usage: prog [options]

Options: -p PATH, --path=<path>  Path to files.", &["-proot"], vec!(("--path", Plain(Some("root".to_string())))))

test_expect!(test_20_testcases, "Usage: prog [options]

Options:    -p --path PATH  Path to files.", &["-p", "root"], vec!(("--path", Plain(Some("root".to_string())))))

test_expect!(test_21_testcases, "Usage: prog [options]

Options:    -p --path PATH  Path to files.", &["--path", "root"], vec!(("--path", Plain(Some("root".to_string())))))

test_expect!(test_22_testcases, "Usage: prog [options]

Options:
 -p PATH  Path to files [default: ./]", &[], vec!(("-p", Plain(Some("./".to_string())))))

test_expect!(test_23_testcases, "Usage: prog [options]

Options:
 -p PATH  Path to files [default: ./]", &["-phome"], vec!(("-p", Plain(Some("home".to_string())))))

test_expect!(test_24_testcases, "UsAgE: prog [options]

OpTiOnS: --path=<files>  Path to files
                [dEfAuLt: /root]", &[], vec!(("--path", Plain(Some("/root".to_string())))))

test_expect!(test_25_testcases, "UsAgE: prog [options]

OpTiOnS: --path=<files>  Path to files
                [dEfAuLt: /root]", &["--path=home"], vec!(("--path", Plain(Some("home".to_string())))))

test_expect!(test_26_testcases, "usage: prog [options]

options:
    -a        Add
    -r        Remote
    -m <msg>  Message", &["-a", "-r", "-m", "Hello"], vec!(("-m", Plain(Some("Hello".to_string()))), ("-a", Switch(true)), ("-r", Switch(true))))

test_expect!(test_27_testcases, "usage: prog [options]

options:
    -a        Add
    -r        Remote
    -m <msg>  Message", &["-armyourass"], vec!(("-m", Plain(Some("yourass".to_string()))), ("-a", Switch(true)), ("-r", Switch(true))))

test_expect!(test_28_testcases, "usage: prog [options]

options:
    -a        Add
    -r        Remote
    -m <msg>  Message", &["-a", "-r"], vec!(("-m", Plain(None)), ("-a", Switch(true)), ("-r", Switch(true))))

test_expect!(test_29_testcases, "Usage: prog [options]

Options: --version
         --verbose", &["--version"], vec!(("--verbose", Switch(false)), ("--version", Switch(true))))

test_expect!(test_30_testcases, "Usage: prog [options]

Options: --version
         --verbose", &["--verbose"], vec!(("--verbose", Switch(true)), ("--version", Switch(false))))

test_user_error!(test_31_testcases, "Usage: prog [options]

Options: --version
         --verbose", &["--ver"])

test_user_error!(test_32_testcases, "Usage: prog [options]

Options: --version
         --verbose", &["--verb"])

test_expect!(test_33_testcases, "usage: prog [-a -r -m <msg>]

options:
 -a        Add
 -r        Remote
 -m <msg>  Message", &["-armyourass"], vec!(("-m", Plain(Some("yourass".to_string()))), ("-a", Switch(true)), ("-r", Switch(true))))

test_expect!(test_34_testcases, "usage: prog [-armMSG]

options: -a        Add
         -r        Remote
         -m <msg>  Message", &["-a", "-r", "-m", "Hello"], vec!(("-m", Plain(Some("Hello".to_string()))), ("-a", Switch(true)), ("-r", Switch(true))))

test_expect!(test_35_testcases, "usage: prog -a -b

options:
 -a
 -b", &["-a", "-b"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_expect!(test_36_testcases, "usage: prog -a -b

options:
 -a
 -b", &["-b", "-a"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_user_error!(test_37_testcases, "usage: prog -a -b

options:
 -a
 -b", &["-a"])

test_user_error!(test_38_testcases, "usage: prog -a -b

options:
 -a
 -b", &[])

test_expect!(test_39_testcases, "usage: prog (-a -b)

options: -a
         -b", &["-a", "-b"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_expect!(test_40_testcases, "usage: prog (-a -b)

options: -a
         -b", &["-b", "-a"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_user_error!(test_41_testcases, "usage: prog (-a -b)

options: -a
         -b", &["-a"])

test_user_error!(test_42_testcases, "usage: prog (-a -b)

options: -a
         -b", &[])

test_expect!(test_43_testcases, "usage: prog [-a] -b

options: -a
 -b", &["-a", "-b"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_expect!(test_44_testcases, "usage: prog [-a] -b

options: -a
 -b", &["-b", "-a"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_user_error!(test_45_testcases, "usage: prog [-a] -b

options: -a
 -b", &["-a"])

test_expect!(test_46_testcases, "usage: prog [-a] -b

options: -a
 -b", &["-b"], vec!(("-a", Switch(false)), ("-b", Switch(true))))

test_user_error!(test_47_testcases, "usage: prog [-a] -b

options: -a
 -b", &[])

test_expect!(test_48_testcases, "usage: prog [(-a -b)]

options: -a
         -b", &["-a", "-b"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_expect!(test_49_testcases, "usage: prog [(-a -b)]

options: -a
         -b", &["-b", "-a"], vec!(("-a", Switch(true)), ("-b", Switch(true))))

test_user_error!(test_50_testcases, "usage: prog [(-a -b)]

options: -a
         -b", &["-a"])

test_user_error!(test_51_testcases, "usage: prog [(-a -b)]

options: -a
         -b", &["-b"])

test_expect!(test_52_testcases, "usage: prog [(-a -b)]

options: -a
         -b", &[], vec!(("-a", Switch(false)), ("-b", Switch(false))))

test_user_error!(test_53_testcases, "usage: prog (-a|-b)

options: -a
         -b", &["-a", "-b"])

test_user_error!(test_54_testcases, "usage: prog (-a|-b)

options: -a
         -b", &[])

test_expect!(test_55_testcases, "usage: prog (-a|-b)

options: -a
         -b", &["-a"], vec!(("-a", Switch(true)), ("-b", Switch(false))))

test_expect!(test_56_testcases, "usage: prog (-a|-b)

options: -a
         -b", &["-b"], vec!(("-a", Switch(false)), ("-b", Switch(true))))

test_user_error!(test_57_testcases, "usage: prog [ -a | -b ]

options: -a
         -b", &["-a", "-b"])

test_expect!(test_58_testcases, "usage: prog [ -a | -b ]

options: -a
         -b", &[], vec!(("-a", Switch(false)), ("-b", Switch(false))))

test_expect!(test_59_testcases, "usage: prog [ -a | -b ]

options: -a
         -b", &["-a"], vec!(("-a", Switch(true)), ("-b", Switch(false))))

test_expect!(test_60_testcases, "usage: prog [ -a | -b ]

options: -a
         -b", &["-b"], vec!(("-a", Switch(false)), ("-b", Switch(true))))

test_expect!(test_61_testcases, "usage: prog <arg>", &["10"], vec!(("<arg>", Plain(Some("10".to_string())))))

test_user_error!(test_62_testcases, "usage: prog <arg>", &["10", "20"])

test_user_error!(test_63_testcases, "usage: prog <arg>", &[])

test_expect!(test_64_testcases, "usage: prog [<arg>]", &["10"], vec!(("<arg>", Plain(Some("10".to_string())))))

test_user_error!(test_65_testcases, "usage: prog [<arg>]", &["10", "20"])

test_expect!(test_66_testcases, "usage: prog [<arg>]", &[], vec!(("<arg>", Plain(None))))

test_expect!(test_67_testcases, "usage: prog <kind> <name> <type>", &["10", "20", "40"], vec!(("<type>", Plain(Some("40".to_string()))), ("<kind>", Plain(Some("10".to_string()))), ("<name>", Plain(Some("20".to_string())))))

test_user_error!(test_68_testcases, "usage: prog <kind> <name> <type>", &["10", "20"])

test_user_error!(test_69_testcases, "usage: prog <kind> <name> <type>", &[])

test_expect!(test_70_testcases, "usage: prog <kind> [<name> <type>]", &["10", "20", "40"], vec!(("<type>", Plain(Some("40".to_string()))), ("<kind>", Plain(Some("10".to_string()))), ("<name>", Plain(Some("20".to_string())))))

test_expect!(test_71_testcases, "usage: prog <kind> [<name> <type>]", &["10", "20"], vec!(("<type>", Plain(None)), ("<kind>", Plain(Some("10".to_string()))), ("<name>", Plain(Some("20".to_string())))))

test_user_error!(test_72_testcases, "usage: prog <kind> [<name> <type>]", &[])

test_user_error!(test_73_testcases, "usage: prog [<kind> | <name> <type>]", &["10", "20", "40"])

test_expect!(test_74_testcases, "usage: prog [<kind> | <name> <type>]", &["20", "40"], vec!(("<type>", Plain(Some("40".to_string()))), ("<kind>", Plain(None)), ("<name>", Plain(Some("20".to_string())))))

test_expect!(test_75_testcases, "usage: prog [<kind> | <name> <type>]", &[], vec!(("<type>", Plain(None)), ("<kind>", Plain(None)), ("<name>", Plain(None))))

test_expect!(test_76_testcases, "usage: prog (<kind> --all | <name>)

options:
 --all", &["10", "--all"], vec!(("--all", Switch(true)), ("<kind>", Plain(Some("10".to_string()))), ("<name>", Plain(None))))

test_expect!(test_77_testcases, "usage: prog (<kind> --all | <name>)

options:
 --all", &["10"], vec!(("--all", Switch(false)), ("<kind>", Plain(None)), ("<name>", Plain(Some("10".to_string())))))

test_user_error!(test_78_testcases, "usage: prog (<kind> --all | <name>)

options:
 --all", &[])

test_expect!(test_79_testcases, "usage: prog [<name> <name>]", &["10", "20"], vec!(("<name>", List(vec!("10".to_string(), "20".to_string())))))

test_expect!(test_80_testcases, "usage: prog [<name> <name>]", &["10"], vec!(("<name>", List(vec!("10".to_string())))))

test_expect!(test_81_testcases, "usage: prog [<name> <name>]", &[], vec!(("<name>", List(vec!()))))

test_expect!(test_82_testcases, "usage: prog [(<name> <name>)]", &["10", "20"], vec!(("<name>", List(vec!("10".to_string(), "20".to_string())))))

test_user_error!(test_83_testcases, "usage: prog [(<name> <name>)]", &["10"])

test_expect!(test_84_testcases, "usage: prog [(<name> <name>)]", &[], vec!(("<name>", List(vec!()))))

test_expect!(test_85_testcases, "usage: prog NAME...", &["10", "20"], vec!(("NAME", List(vec!("10".to_string(), "20".to_string())))))

test_expect!(test_86_testcases, "usage: prog NAME...", &["10"], vec!(("NAME", List(vec!("10".to_string())))))

test_user_error!(test_87_testcases, "usage: prog NAME...", &[])

test_expect!(test_88_testcases, "usage: prog [NAME]...", &["10", "20"], vec!(("NAME", List(vec!("10".to_string(), "20".to_string())))))

test_expect!(test_89_testcases, "usage: prog [NAME]...", &["10"], vec!(("NAME", List(vec!("10".to_string())))))

test_expect!(test_90_testcases, "usage: prog [NAME]...", &[], vec!(("NAME", List(vec!()))))

test_expect!(test_91_testcases, "usage: prog [NAME...]", &["10", "20"], vec!(("NAME", List(vec!("10".to_string(), "20".to_string())))))

test_expect!(test_92_testcases, "usage: prog [NAME...]", &["10"], vec!(("NAME", List(vec!("10".to_string())))))

test_expect!(test_93_testcases, "usage: prog [NAME...]", &[], vec!(("NAME", List(vec!()))))

test_expect!(test_94_testcases, "usage: prog [NAME [NAME ...]]", &["10", "20"], vec!(("NAME", List(vec!("10".to_string(), "20".to_string())))))

test_expect!(test_95_testcases, "usage: prog [NAME [NAME ...]]", &["10"], vec!(("NAME", List(vec!("10".to_string())))))

test_expect!(test_96_testcases, "usage: prog [NAME [NAME ...]]", &[], vec!(("NAME", List(vec!()))))

test_expect!(test_97_testcases, "usage: prog (NAME | --foo NAME)

options: --foo", &["10"], vec!(("NAME", Plain(Some("10".to_string()))), ("--foo", Switch(false))))

test_expect!(test_98_testcases, "usage: prog (NAME | --foo NAME)

options: --foo", &["--foo", "10"], vec!(("NAME", Plain(Some("10".to_string()))), ("--foo", Switch(true))))

test_user_error!(test_99_testcases, "usage: prog (NAME | --foo NAME)

options: --foo", &["--foo=10"])

test_expect!(test_100_testcases, "usage: prog (NAME | --foo) [--bar | NAME]

options: --foo
options: --bar", &["10"], vec!(("--bar", Switch(false)), ("NAME", List(vec!("10".to_string()))), ("--foo", Switch(false))))

test_expect!(test_101_testcases, "usage: prog (NAME | --foo) [--bar | NAME]

options: --foo
options: --bar", &["10", "20"], vec!(("--bar", Switch(false)), ("NAME", List(vec!("10".to_string(), "20".to_string()))), ("--foo", Switch(false))))

test_expect!(test_102_testcases, "usage: prog (NAME | --foo) [--bar | NAME]

options: --foo
options: --bar", &["--foo", "--bar"], vec!(("--bar", Switch(true)), ("NAME", List(vec!())), ("--foo", Switch(true))))

test_expect!(test_103_testcases, "Naval Fate.

Usage:
  prog ship new <name>...
  prog ship [<name>] move <x> <y> [--speed=<kn>]
  prog ship shoot <x> <y>
  prog mine (set|remove) <x> <y> [--moored|--drifting]
  prog -h | --help
  prog --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --speed=<kn>  Speed in knots [default: 10].
  --moored      Mored (anchored) mine.
  --drifting    Drifting mine.", &["ship", "Guardian", "move", "150", "300", "--speed=20"], vec!(("shoot", Switch(false)), ("--moored", Switch(false)), ("--drifting", Switch(false)), ("move", Switch(true)), ("--speed", Plain(Some("20".to_string()))), ("mine", Switch(false)), ("new", Switch(false)), ("--version", Switch(false)), ("set", Switch(false)), ("remove", Switch(false)), ("<name>", List(vec!("Guardian".to_string()))), ("ship", Switch(true)), ("<x>", Plain(Some("150".to_string()))), ("<y>", Plain(Some("300".to_string()))), ("--help", Switch(false))))

test_expect!(test_104_testcases, "usage: prog --hello", &["--hello"], vec!(("--hello", Switch(true))))

test_expect!(test_105_testcases, "usage: prog [--hello=<world>]", &[], vec!(("--hello", Plain(None))))

test_expect!(test_106_testcases, "usage: prog [--hello=<world>]", &["--hello", "wrld"], vec!(("--hello", Plain(Some("wrld".to_string())))))

test_expect!(test_107_testcases, "usage: prog [-o]", &[], vec!(("-o", Switch(false))))

test_expect!(test_108_testcases, "usage: prog [-o]", &["-o"], vec!(("-o", Switch(true))))

test_expect!(test_109_testcases, "usage: prog [-opr]", &["-op"], vec!(("-o", Switch(true)), ("-p", Switch(true)), ("-r", Switch(false))))

test_expect!(test_110_testcases, "usage: prog --aabb | --aa", &["--aa"], vec!(("--aa", Switch(true)), ("--aabb", Switch(false))))

test_user_error!(test_111_testcases, "usage: prog --aabb | --aa", &["--a"])

test_expect!(test_112_testcases, "Usage: prog -v", &["-v"], vec!(("-v", Switch(true))))

test_expect!(test_113_testcases, "Usage: prog [-v -v]", &[], vec!(("-v", Counted(0))))

test_expect!(test_114_testcases, "Usage: prog [-v -v]", &["-v"], vec!(("-v", Counted(1))))

test_expect!(test_115_testcases, "Usage: prog [-v -v]", &["-vv"], vec!(("-v", Counted(2))))

test_user_error!(test_116_testcases, "Usage: prog -v ...", &[])

test_expect!(test_117_testcases, "Usage: prog -v ...", &["-v"], vec!(("-v", Counted(1))))

test_expect!(test_118_testcases, "Usage: prog -v ...", &["-vv"], vec!(("-v", Counted(2))))

test_expect!(test_119_testcases, "Usage: prog -v ...", &["-vvvvvv"], vec!(("-v", Counted(6))))

test_expect!(test_120_testcases, "Usage: prog [-v | -vv | -vvv]

This one is probably most readable user-friednly variant.", &[], vec!(("-v", Counted(0))))

test_expect!(test_121_testcases, "Usage: prog [-v | -vv | -vvv]

This one is probably most readable user-friednly variant.", &["-v"], vec!(("-v", Counted(1))))

test_expect!(test_122_testcases, "Usage: prog [-v | -vv | -vvv]

This one is probably most readable user-friednly variant.", &["-vv"], vec!(("-v", Counted(2))))

test_user_error!(test_123_testcases, "Usage: prog [-v | -vv | -vvv]

This one is probably most readable user-friednly variant.", &["-vvvv"])

test_expect!(test_124_testcases, "usage: prog [--ver --ver]", &["--ver", "--ver"], vec!(("--ver", Counted(2))))

test_expect!(test_125_testcases, "usage: prog [go]", &["go"], vec!(("go", Switch(true))))

test_expect!(test_126_testcases, "usage: prog [go go]", &[], vec!(("go", Counted(0))))

test_expect!(test_127_testcases, "usage: prog [go go]", &["go"], vec!(("go", Counted(1))))

test_expect!(test_128_testcases, "usage: prog [go go]", &["go", "go"], vec!(("go", Counted(2))))

test_user_error!(test_129_testcases, "usage: prog [go go]", &["go", "go", "go"])

test_expect!(test_130_testcases, "usage: prog go...", &["go", "go", "go", "go", "go"], vec!(("go", Counted(5))))

test_expect!(test_131_testcases, "usage: prog [options] [-a]

options: -a
         -b", &["-a"], vec!(("-a", Switch(true)), ("-b", Switch(false))))

test_user_error!(test_132_testcases, "usage: prog [options] [-a]

options: -a
         -b", &["-aa"])

test_expect!(test_133_testcases, "Usage: prog [options] A

Options:
    -q  Be quiet
    -v  Be verbose.", &["arg"], vec!(("A", Plain(Some("arg".to_string()))), ("-v", Switch(false)), ("-q", Switch(false))))

test_expect!(test_134_testcases, "Usage: prog [options] A

Options:
    -q  Be quiet
    -v  Be verbose.", &["-v", "arg"], vec!(("A", Plain(Some("arg".to_string()))), ("-v", Switch(true)), ("-q", Switch(false))))

test_expect!(test_135_testcases, "Usage: prog [options] A

Options:
    -q  Be quiet
    -v  Be verbose.", &["-q", "arg"], vec!(("A", Plain(Some("arg".to_string()))), ("-v", Switch(false)), ("-q", Switch(true))))

test_expect!(test_136_testcases, "usage: prog [-]", &["-"], vec!(("-", Switch(true))))

test_expect!(test_137_testcases, "usage: prog [-]", &[], vec!(("-", Switch(false))))

test_expect!(test_138_testcases, "usage: prog [NAME [NAME ...]]", &["a", "b"], vec!(("NAME", List(vec!("a".to_string(), "b".to_string())))))

test_expect!(test_139_testcases, "usage: prog [NAME [NAME ...]]", &[], vec!(("NAME", List(vec!()))))

test_expect!(test_140_testcases, "usage: prog [options]

options:
 -a        Add
 -m <msg>  Message", &["-a"], vec!(("-m", Plain(None)), ("-a", Switch(true))))

test_expect!(test_141_testcases, "usage: prog --hello", &["--hello"], vec!(("--hello", Switch(true))))

test_expect!(test_142_testcases, "usage: prog [--hello=<world>]", &[], vec!(("--hello", Plain(None))))

test_expect!(test_143_testcases, "usage: prog [--hello=<world>]", &["--hello", "wrld"], vec!(("--hello", Plain(Some("wrld".to_string())))))

test_expect!(test_144_testcases, "usage: prog [-o]", &[], vec!(("-o", Switch(false))))

test_expect!(test_145_testcases, "usage: prog [-o]", &["-o"], vec!(("-o", Switch(true))))

test_expect!(test_146_testcases, "usage: prog [-opr]", &["-op"], vec!(("-o", Switch(true)), ("-p", Switch(true)), ("-r", Switch(false))))

test_expect!(test_147_testcases, "usage: git [-v | --verbose]", &["-v"], vec!(("-v", Switch(true)), ("--verbose", Switch(false))))

test_expect!(test_148_testcases, "usage: git remote [-v | --verbose]", &["remote", "-v"], vec!(("-v", Switch(true)), ("remote", Switch(true)), ("--verbose", Switch(false))))

test_expect!(test_149_testcases, "usage: prog", &[], vec!())

test_expect!(test_150_testcases, "usage: prog
           prog <a> <b>", &["1", "2"], vec!(("<a>", Plain(Some("1".to_string()))), ("<b>", Plain(Some("2".to_string())))))

test_expect!(test_151_testcases, "usage: prog
           prog <a> <b>", &[], vec!(("<a>", Plain(None)), ("<b>", Plain(None))))

test_expect!(test_152_testcases, "usage: prog <a> <b>
           prog", &[], vec!(("<a>", Plain(None)), ("<b>", Plain(None))))

test_expect!(test_153_testcases, "usage: prog [--file=<f>]", &[], vec!(("--file", Plain(None))))

test_expect!(test_154_testcases, "usage: prog [--file=<f>]

options: --file <a>", &[], vec!(("--file", Plain(None))))

test_expect!(test_155_testcases, "Usage: prog [-a <host:port>]

Options: -a, --address <host:port>  TCP address [default: localhost:6283].", &[], vec!(("--address", Plain(Some("localhost:6283".to_string())))))

test_expect!(test_156_testcases, "usage: prog --long=<arg> ...", &["--long", "one"], vec!(("--long", List(vec!("one".to_string())))))

test_expect!(test_157_testcases, "usage: prog --long=<arg> ...", &["--long", "one", "--long", "two"], vec!(("--long", List(vec!("one".to_string(), "two".to_string())))))

test_expect!(test_158_testcases, "usage: prog (go <direction> --speed=<km/h>)...", &["go", "left", "--speed=5", "go", "right", "--speed=9"], vec!(("go", Counted(2)), ("<direction>", List(vec!("left".to_string(), "right".to_string()))), ("--speed", List(vec!("5".to_string(), "9".to_string())))))

test_expect!(test_159_testcases, "usage: prog [options] -a

options: -a", &["-a"], vec!(("-a", Switch(true))))

test_expect!(test_160_testcases, "usage: prog [-o <o>]...

options: -o <o>  [default: x]", &["-o", "this", "-o", "that"], vec!(("-o", List(vec!("this".to_string(), "that".to_string())))))

test_expect!(test_161_testcases, "usage: prog [-o <o>]...

options: -o <o>  [default: x]", &[], vec!(("-o", List(vec!("x".to_string())))))

test_expect!(test_162_testcases, "usage: prog [-o <o>]...

options: -o <o>  [default: x y]", &["-o", "this"], vec!(("-o", List(vec!("this".to_string())))))

test_expect!(test_163_testcases, "usage: prog [-o <o>]...

options: -o <o>  [default: x y]", &[], vec!(("-o", List(vec!("x".to_string(), "y".to_string())))))

test_expect!(test_164_testcases, "usage: prog -pPATH

options: -p PATH", &["-pHOME"], vec!(("-p", Plain(Some("HOME".to_string())))))

test_expect!(test_165_testcases, "Usage: foo (--xx=X|--yy=Y)...", &["--xx=1", "--yy=2"], vec!(("--yy", List(vec!("2".to_string()))), ("--xx", List(vec!("1".to_string())))))

test_expect!(test_166_testcases, "usage: prog [<input file>]", &["f.txt"], vec!(("<input file>", Plain(Some("f.txt".to_string())))))

test_expect!(test_167_testcases, "usage: prog [--input=<file name>]...", &["--input", "a.txt", "--input=b.txt"], vec!(("--input", List(vec!("a.txt".to_string(), "b.txt".to_string())))))

test_expect!(test_168_testcases, "usage: prog good [options]
           prog fail [options]

options: --loglevel=N", &["fail", "--loglevel", "5"], vec!(("fail", Switch(true)), ("good", Switch(false)), ("--loglevel", Plain(Some("5".to_string())))))

test_expect!(test_169_testcases, "usage:prog --foo", &["--foo"], vec!(("--foo", Switch(true))))

test_expect!(test_170_testcases, "PROGRAM USAGE: prog --foo", &["--foo"], vec!(("--foo", Switch(true))))

test_expect!(test_171_testcases, "Usage: prog --foo
           prog --bar
NOT PART OF SECTION", &["--foo"], vec!(("--bar", Switch(false)), ("--foo", Switch(true))))

test_expect!(test_172_testcases, "Usage:
 prog --foo
 prog --bar

NOT PART OF SECTION", &["--foo"], vec!(("--bar", Switch(false)), ("--foo", Switch(true))))

test_expect!(test_173_testcases, "Usage:
 prog --foo
 prog --bar
NOT PART OF SECTION", &["--foo"], vec!(("--bar", Switch(false)), ("--foo", Switch(true))))

test_expect!(test_174_testcases, "Usage: prog [options]

global options: --foo
local options: --baz
               --bar
other options:
 --egg
 --spam
-not-an-option-", &["--bar", "--egg"], vec!(("--bar", Switch(true)), ("--egg", Switch(true)), ("--spam", Switch(false))))

test_expect!(test_175_testcases, "Usage: prog [-a] [--] [<arg>...]", &["-a"], vec!(("--", Switch(false)), ("<arg>", List(vec!())), ("-a", Switch(true))))

test_expect!(test_176_testcases, "Usage: prog [-a] [--] [<arg>...]", &["--"], vec!(("--", Switch(true)), ("<arg>", List(vec!())), ("-a", Switch(false))))

test_expect!(test_177_testcases, "Usage: prog [-a] [--] [<arg>...]", &["-a", "--", "-b"], vec!(("--", Switch(true)), ("<arg>", List(vec!("-b".to_string()))), ("-a", Switch(true))))

test_expect!(test_178_testcases, "Usage: prog [-a] [--] [<arg>...]", &["-a", "--", "-a"], vec!(("--", Switch(true)), ("<arg>", List(vec!("-a".to_string()))), ("-a", Switch(true))))

test_expect!(test_179_testcases, "Usage: prog [-a] [--] [<arg>...]", &["--", "-a"], vec!(("--", Switch(true)), ("<arg>", List(vec!("-a".to_string()))), ("-a", Switch(false))))

