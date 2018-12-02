fn solve(input: &str) {
    let mut close_lines: Vec<&str> = Vec::new();

    for a in input.lines() {
        for b in input.lines() {
            if char_diff_num(a, b) == 1 {
                if !close_lines.contains(&a) || !close_lines.contains(&b){
                    close_lines.push(a);
                    close_lines.push(b);
                }
            }
        } 
    }

    char_diff(close_lines[0], close_lines[1]);
    // println!("{:#?}", close_lines);
}

fn char_diff_num(l1: &str, l2: &str) -> u32 {
    let len = if l1.len() > l2.len() { l1.len() } else { l2.len() };
    let mut diff = 0;
    
    let mut l1 = l1.chars();
    let mut l2 = l2.chars();

    for _i in 0..len {
        if l1.next() != l2.next() {
            diff += 1;
        }
    }

    diff
}

fn char_diff(l1: &str, l2: &str) {
    let len = if l1.len() > l2.len() { l1.len() } else { l2.len() };
    let mut diff_index = 0;

    let mut l1_iter = l1.chars();
    let mut l2_iter = l2.chars();

    for i in 0..len {
        if l1_iter.next() != l2_iter.next() {
            diff_index = i;
            break;
        }
    }

    println!("{}", diff_index);
    println!("{}", l1);
    println!("{}", l2);
    println!("{}{}", l1.get(0..diff_index).unwrap(), l1.get((diff_index + 1)..).unwrap());

    
}

const INPUT: &str = "zihrtxagncfpbsnolxydujjmqv
zihrtxagwcfpbsoolnydukjyqv
aihrtxagwcfpbsnoleybmkjmqv
zihrtxagwcfpbsnolgyduajmrv
zihrtxgmwcfpbunoleydukjmqv
zihqtxagwcfpbsnolesdukomqv
zihgtxagwcfpbsnoleydqkjqqv
dihrtxagwcqpbsnoleydpkjmqv
qihrtvagwcfpbsnollydukjmqv
zihrtgagwcfpbknoleyrukjmqv
cinrtxagwcfpbsnoleydukjaqv
zihrtxagwcfubsneleyvukjmqv
zihrtxagwcfpbsvoleydukvmtv
zihrtpagwcffbsnolfydukjmqv
zihrtxagwcfpbsxoleydtkjyqv
zohrvxugwcfpbsnoleydukjmqv
zyhrtxagdcfpbsnodeydukjmqv
zihrtxaghffpbsnoleyduojmqv
oihrtbagwcfpbsnoleyduejmqv
zihrtnagwcvpjsnoleydukjmqv
iihrtxagwcfpbsnoliyaukjmqv
ziartxagwcfpbsnokeydukjmpv
eibrtxagwccpbsnoleydukjmqv
zihrtxagwczwbsaoleydukjmqv
ziiatuagwcfpbsnoleydukjmqv
zzhrtxagwckpbsnsleydukjmqv
cihrtxaqwcfpbsnoleydkkjmqv
zihrtxaywcfpbsnoleydukzdqv
zihrtxagwjfpbvnoleydukjmql
zihrtxagwcfpbsnoleuduksmql
zizrtxxgwcfpbsnoleydukzmqv
zihrteagwcfpbsnobeydukjmqe
zihrtxafwhfpbsgoleydukjmqv
zitrtxagwcfpbsnoleyduvymqv
zihrtxauwcfebsnoleygukjmqv
zihrtxagwcfpbsnoleydubjrqh
zihrtxauwmfpbsnoleydukjmqo
zihrtxagwcdpbsnoleydukxmov
zihrtmagwcfpbsnoleydukvmlv
ziwrtxhgwcfpbsnoleodukjmqv
zihytxagacfpbsnoceydukjmqv
zihrtxagwcfpbsnolebdugjnqv
zihrzxagwcfpbsnjleyduktmqv
zihrtxygwcfpbinoleysukjmqv
zihrtxagwcfpbmnoveydujjmqv
zidrtxagwcfpbsnolexaukjmqv
zshrtxagwcepbsnoxeydukjmqv
yibrtxagwzfpbsnoleydukjmqv
zehrtxagwclpbsnoleymukjmqv
zihruxagwcfpbsnoleyhukwmqv
zihrwxagwcfpbszolesdukjmqv
zihrtpagwcfpbwnoleyuukjmqv
ziortxagwcfpssnolewdukjmqv
zohrtxagwcfpbwnoleydukjmjv
zihrtxagwcfpbsnvleyduzcmqv
zihrvxaghcfpbswoleydukjmqv
zihrtxagwcfpssnolwydukzmqv
zjhrttagwcfpbsnolfydukjmqv
zihrtxagwjfpbsnoljydukpmqv
ziwrtxagwczpbsnoljydukjmqv
zinrtxagwcfpbvfoleydukjmqv
zihrgragwcfpbsnoleydutjmqv
zihrtxagwcfpbsnozeydukffqv
zihrtxagwcfpbsmoleydxkumqv
rihwtxagwcfpbsxoleydukjmqv
ziqrtxagwcfpbsnqlevdukjmqv
zihrtxagwchpbsnoleydufamqv
sihrtxagwcfpbsnoleldukjmqp
zihrtxagwcrpbsnoleydvojmqv
zihrtxacwcfpbsnoweyxukjmqv
zihrtxagwcfpbsnolajmukjmqv
zzfrtxagwcfpbsnoleydukjmvv
zixrtxagwcfpbqnoleydukjgqv
zihitxaqwcfpbsnoleadukjmqv
zilrtxagecfxbsnoleydukjmqv
zihrtxagwcfpbypoleycukjmqv
zidrtxagdtfpbsnoleydukjmqv
lehrtxagxcfpbsnoleydukjmqv
zihrlxagwcfpbsncneydukjmqv
zihroxagbcspbsnoleydukjmqv
zihrtxagwcfkzsnolemdukjmqv
zihrtxagwcfpbsqeleydukkmqv
zihrjxagwcfpesnolxydukjmqv
zifrtxagwcfpbsooleydukkmqv
zirwtxagwcfpbsnoleydukzmqv
zjhntxagwcfpbsnoleydunjmqv
ziorexagwcfpbsnoyeydukjmqv
zhhrtlagwcfybsnoleydukjmqv
zirrtxagwvfsbsnoleydukjmqv
bihrtxagwofpbsnoleadukjmqv
dihrtxagwcfpksnoleydukjlqv
zihrrxagecfpbsnoleydukjmyv
zijrtxagwmfpbsnoleyduljmqv
zihrtxagwcfpbsnolecdukjpqs
zchrtxagwcfpbsnolehdukjmwv
rmhrtxagwcfpbsnoleydkkjmqv
zohrotagwcfpbsnoleydukjmqv
zihwtxagsifpbsnwleydukjmqv
zihrtxagicfpbsnoleydukjxqn
zihrtxsgwcfpbsntleydumjmqv
zihrlxagzgfpbsnoleydukjmqv
aihjtxagwdfpbsnoleydukjmqv
zifrtxagwcfhbsnoleddukjmqv
zihrtyagwcfpbsooleydtkjmqv
zihrtxxgwcfpbsnolerhukjmqv
zihqtxalwcfppsnoleydukjmqv
zfkrvxagwcfpbsnoleydukjmqv
zihptxagwcfpbseoleydukjmdv
zihrtxagwcfpeonoleyiukjmqv
nidrtxagwcfpbsnoleyhukjmqv
zihrtxagwcfjbsnolsydukjmqg
zghryxagwcfgbsnoleydukjmqv
zihwtxagwcfpbsnoleydugjfqv
zihryxagwjfpbsnoleydujjmqv
zihrtxagwcfpbsnolekdukymql
zfhrtxaownfpbsnoleydukjmqv
zamrtxagwcfpbsnoleyduzjmqv
ibhrtxagwcfpbsnoleydukjmfv
zihrtxagwcfpssnoseydukjmuv
zihrtxagwcfpbsnoljydukjhqs
zihrtxagwqfmbsnoleidukjmqv
zfdrtxagwchpbsnoleydukjmqv
iihrtxagqcfpbsnoleydukjmqn
mihrtxagwcfpbsqoleydukjbqv
zihttxagwcfpbsnoleyduljmqk
zzhrtxagwcfpzseoleydukjmqv
zdhrtxagbcfpbsnoleyduyjmqv
zihxtxagwcfpbsnolwrdukjmqv
zghrtxagwcypbynoleydukjmqv
zihrtxaiwcfppsnoleydukgmqv
zitatxagwcfobsnoleydukjmqv
znhrtxagwcfpysnoleydukjqqv
zihrtxagwcfppsnoleoyukjmqv
ziorgxagwcfpbsnolekdukjmqv
zihrtxagwcfpbfnoleydwkjpqv
zihrtxnrwcfpbsnolnydukjmqv
rihrtxagwcfpbsnolepdjkjmqv
zihrtxagwcfzbsnoceydukjmkv
zihrtxagwcfpysnoaeidukjmqv
zihrmxagwcfpbsnoleydukjmuq
gihrtxagwcvpbsnoleydukcmqv
zihrtxagocfpbsnoleydukqmnv
zihrtxagwcfpesnoleyluklmqv
zghrtxagwcfzbsnoleydukjmgv
zihrtxugqqfpbsnoleydukjmqv
zirrtcagwcfpbsnoleydfkjmqv
zihitxagwcfpjsnoleydnkjmqv
zihrtxqgwcfpbsnsleydukjmqy
iihrtxagwyfpbsnoleydukjmqu
zihrsxagwcfpbsnsleydukzmqv
zihrtxawwcfpbsnoleydzkjmuv
dihrkxagwcfpbsfoleydukjmqv
zihrtxaqwcfpbvnoleydukjmqt
zihntxdgwcfpbsnogeydukjmqv
zihrtxagwcdpxsnolxydukjmqv
zihrtxagwcfpbsaoleydunjaqv
zihrtyagwcfpbsnoleyduqjmqt
zihrtxagwtfpbsnoleoyukjmqv
zihrjiagwcfpbsnobeydukjmqv
zihrtxqgwcfpbsnoleydykdmqv
zihrhxmgwcfpbsnmleydukjmqv
zihatxlgwcfpbsnoleydukpmqv
zihrtxcgwcspbsnoleypukjmqv
zihrtkagqcfpbsaoleydukjmqv
ziqrtxagwcfabsnoleydukrmqv
zihwtxagwifpbsnwleydukjmqv
zitrtnagwcfpbsnoleddukjmqv
wihrtxagwcfpbsioyeydukjmqv
zihrtxagwclpystoleydukjmqv
zihmtxagwcfpbsnolfydukjmlv
zihrtxagechpbsnoleydutjmqv
zihrtxagwcfebsnolnydukjmuv
zihrtxagncmpbsnoleydukjmqs
zihrvxagocfpbsnoleydukcmqv
zihrtxagwcjcbsnolejdukjmqv
wihrtxagwcfpbogoleydukjmqv
kivrtxagwcfpgsnoleydukjmqv
zihrtxagwafpbhnoleydukjcqv
zihrtwagtcfpbsnolxydukjmqv
vihrtxagwcfpbsneletdukjmqv
zihlnxagwcfpbsnoleydukjmqb
zihrtxagwcfpbsnoleydukjuuc
zihrtxagwcfpbwntleadukjmqv
fihrtxagwcfpbsnoleydvkjmqw
zihrtxaowcfpbunoleyduljmqv
zthrtxagwcfpbtnoleydukomqv
xihltxagwcfpbsnoleydukjrqv
ziyrnxagwcfpbsnoleydukjmhv
zihrtxazwcfpbsnileyduejmqv
zihrtxagwcfibsnoliydukjmsv
zihrtxggwcfpbsnoleydugjmqj
zrartxagwcffbsnoleydukjmqv
zidrtxaqwcfpbsnoleyduksmqv
zirrtxagwcypbsnoleydtkjmqv
rihrtxagwcrpbsnoheydukjmqv
zihrtxagwcfpbsnoleydpkjmzs
zihrtxagbcfpbsnodbydukjmqv
fihrtxaqwcfpbsnolaydukjmqv
vihrtxbgwcfpbsnolemdukjmqv
zihrtxapwcfubsnoleydukmmqv
zihrtxagwcfpbgnolfydunjmqv
zihrtxagwcypbsnokeyduvjmqv
zihntxagwcfpbsnoieydukbmqv
zihbtxagwkfpbsnolpydukjmqv
zihrtxagwcfibsnoleydikjmqb
jihrtxvgwcfpbsnoleydukjmqp
zihrtxagwcfpbjnqleydukjmlv
zibrtxagwcfpbzvoleydukjmqv
zihrtxagwafgbsnbleydukjmqv
zihjctagwcfpbsnoleydukjmqv
zahrtxagwcepbsnoleddukjmqv
zihetxagwcfpbsnoleydumjmsv
zihrtvagwcfpbbnoleydukdmqv
zbhrxxagwkfpbsnoleydukjmqv
jfhrtxagwcftbsnoleydukjmqv
yihrtxagwcfvbsnoleyduksmqv
ziartxaewcfpbsnoleyduhjmqv
zihrtxagwcfpbsnoozyduzjmqv
cihotxagwcfpysnoleydukjmqv
zihrtxagwcfpusnolwydxkjmqv
zihrtxagwcfpbsnoleedmgjmqv
zihrtxaghcfpmsnoleydukqmqv
ziortxagwcfpbsboleidukjmqv
zihrtxagwcfybsnoleyqxkjmqv
zihrtxamwcfpbsngleydukjmqx
zihrtxagwcfpbsnoleyduusmqu
zihftxagwcfpssnwleydukjmqv
zihrtxagwcfkbsnomeydukjmsv
zihrtxagwcvpbsnooeydwkjmqv
zihrtxagwcfpbsnoleycekumqv
jahrtxagwcfpbsnoleydukjmmv
zihrtxabwcfpbsnzheydukjmqv
zihrtxagwctpbsnoleydwkjmhv
zihrtpagwcfpbsnoleydzkjmqh
zihwtxagwcfpbsnollydukjrqv
zihrtxagwcfpusnoleydsvjmqv
zibrtxagwcfpasnoleydukjmbv
zchrtmagwcfpbsnoleydukjmwv
ziertxbgwyfpbsnoleydukjmqv
zitrtxagwcfpbhnoweydukjmqv
zisrtxkgwcfpbsnopeydukjmqv
zihrtxcgwdfpbynoleydukjmqv
iihrtxajwcvpbsnoleydukjmqv
zihuwxapwcfpbsnoleydukjmqv
zihrtxngwcfqbsnoleyiukjmqv
ziqrtxagjcfpbsnoleydukjmqi
zifrtxarwctpbsnoleydukjmqv
zihxgxagwcfpbpnoleydukjmqv
giprtxagwcdpbsnoleydukjmqv
zihrtxagwmfpbsnodeydukjbqv";

fn main() {
    solve(INPUT);
}

