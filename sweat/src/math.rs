#![allow(clippy::excessive_precision, clippy::unreadable_literal)]

const TEST_DATA_LEN: usize = 400;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn formula(steps_since_tge: f64, steps: f64) -> u128 {
    let trillion = (steps_since_tge / 1e+12).floor() as usize;
    if trillion < KS.len() {
        (area_under_line(KS[trillion], BS[trillion], steps_since_tge, steps_since_tge + steps) * 1e+18) as u128
    } else {
        (exp_decay(steps_since_tge, steps) * 1e+18) as u128
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn area_under_line(k: f64, b: f64, x_start: f64, x_end: f64) -> f64 {
    let square_area = (k * x_end + b) * (x_end - x_start);
    let triangle_area = ((k * x_start + b) - (k * x_end + b)) * (x_end - x_start) / 2.;

    square_area + triangle_area
}

pub fn exp_decay(steps_from_tge: f64, steps_to_exchange: f64) -> f64 {
    ((K * (steps_from_tge + steps_to_exchange) + 1000.).ln() - (K * steps_from_tge + 1000.).ln()) / K
}

pub const K: f64 = 0.00000000084;

// one line per trillion, y = KS[i] * x + BS[i]
pub const KS: [f64; TEST_DATA_LEN] = [
    -4.6195652173913043793e-16,
    -1.6864049318624269812e-16,
    -8.8152985074626865086e-17,
    -5.4185779816513761163e-17,
    -3.6679604798870853723e-17,
    -2.647733061640346396e-17,
    -2.0011935931002618118e-17,
    -1.5657006868297385164e-17,
    -1.25841363614352815456e-17,
    -1.0335056671306422701e-17,
    -8.639461436170212733e-18,
    -7.329506994584837517e-18,
    -6.2964892302473771925e-18,
    -5.467484378616061084e-18,
    -4.792089249492900589e-18,
    -4.2345608603552224086e-18,
    -3.7689808704732346046e-18,
    -3.3761838566770166122e-18,
    -3.0417505501193875931e-18,
    -2.7546639813440746164e-18,
    -2.506389545257269611e-18,
    -2.2902327466929876274e-18,
    -2.1008827951947484923e-18,
    -1.9340830269562240405e-18,
    -1.7863894139886578375e-18,
    -1.6549912434325744265e-18,
    -1.5375763241349931305e-18,
    -1.4322285172611436868e-18,
    -1.3373490255813833786e-18,
    -1.2515953476051725358e-18,
    -1.1738335064817742409e-18,
    -1.103100354010849534e-18,
    -1.0385735923556187876e-18,
    -9.795477589605768502e-19,
    -9.254148564917028697e-19,
    -8.756486286137879879e-19,
    -8.2979171788837764683e-19,
    -7.874451174615102528e-19,
    -7.482594599575014517e-19,
    -7.119277867572528269e-19,
    -6.7817951695611894874e-19,
    -6.467753937894629393e-19,
    -6.175032315705432811e-19,
    -5.901743214272737175e-19,
    -5.6462038173660825646e-19,
    -5.4069096091628782634e-19,
    -5.1825121747905058506e-19,
    -4.9718001599393913773e-19,
    -4.7736828860139127754e-19,
    -4.587176205816159903e-19,
    -4.411390256323204896e-19,
    -4.2455188232295839067e-19,
    -4.088830079314256587e-19,
    -3.9406584974926872992e-19,
    -3.8003977713107442156e-19,
    -3.6674946019559970864e-19,
    -3.541443232658000506e-19,
    -3.421780629462798524e-19,
    -3.3080822224711549269e-19,
    -3.1999581342658720344e-19,
    -3.097049832857628047e-19,
    -2.999027155404711424e-19,
    -2.9055856564974429634e-19,
    -2.8164442411775798466e-19,
    -2.7313430482786889643e-19,
    -2.6500415542831503597e-19,
    -2.5723168718251250153e-19,
    -2.497962220333965158e-19,
    -2.4267855491982619494e-19,
    -2.3586082963110867015e-19,
    -2.2932642669937610756e-19,
    -2.2305986201403206454e-19,
    -2.170466950021005176e-19,
    -2.1127344535669949211e-19,
    -2.0572751741608613105e-19,
    -2.0039713140037014175e-19,
    -1.9527126080425835883e-19,
    -1.9033957532393433473e-19,
    -1.855923887659739793e-19,
    -1.810206114473986654e-19,
    -1.7661570664972152603e-19,
    -1.7236965073713520616e-19,
    -1.6827489659066479677e-19,
    -1.6432434004689290244e-19,
    -1.6051128906237936496e-19,
    -1.5682943535368469915e-19,
    -1.5327282828842835149e-19,
    -1.498358508254737379e-19,
    -1.4651319732248012684e-19,
    -1.4329985304700014566e-19,
    -1.4019107524329456406e-19,
    -1.3718237562131277029e-19,
    -1.342695041470482917e-19,
    -1.314484340248992039e-19,
    -1.2871534777289634843e-19,
    -1.2606662430084262743e-19,
    -1.23498826909651846e-19,
    -1.2100869213758973447e-19,
    -1.1859311938579374149e-19,
    -1.1624916126146275974e-19,
    -1.1397401458253385229e-19,
    -1.1176501199256319127e-19,
    -1.0961961413895823002e-19,
    -1.07535402371716834336e-19,
    -1.05510071923460282696e-19,
    -1.03541425534839524406e-19,
    -1.0162736749238210123e-19,
    -9.9765898048561758293e-20,
    -9.795510819634067199e-20,
    -9.6193174772680584445e-20,
    -9.447835586756488332e-20,
    -9.2808986516939001874e-20,
    -9.118347465967818429e-20,
    -8.96002973402459884e-20,
    -8.805799714012700054e-20,
    -8.6555178822416582764e-20,
    -8.5090506175139591764e-20,
    -8.366269903995946118e-20,
    -8.2270530513937949674e-20,
    -8.0912824312922077287e-20,
    -7.9588452285976353404e-20,
    -7.8296332071051341355e-20,
    -7.7035424882790544804e-20,
    -7.58047334240316256e-20,
    -7.4603299913160179886e-20,
    -7.343020422002924414e-20,
    -7.2284562103669204745e-20,
    -7.1165523545484867336e-20,
    -7.00722711720721516e-20,
    -6.900401876218938202e-20,
    -6.7960009832790263406e-20,
    -6.6939516299369744327e-20,
    -6.5941837206192481045e-20,
    -6.49662975222685348e-20,
    -6.4012246999214240833e-20,
    -6.307905908738952739e-20,
    -6.2166129906937991274e-20,
    -6.127287727057418898e-20,
    -6.039873975516518142e-20,
    -5.954317581934159352e-20,
    -5.870566296454844333e-20,
    -5.7885696937108791174e-20,
    -5.70827909690247192e-20,
    -5.6296475055381217727e-20,
    -5.5526295266349942987e-20,
    -5.4771813091912319165e-20,
    -5.4032604817535609017e-20,
    -5.330826092914221942e-20,
    -5.2598385545811852415e-20,
    -5.19025958787491446e-20,
    -5.1220521715136045092e-20,
    -5.0551804925569458667e-20,
    -4.9896098993860403153e-20,
    -4.9253068568042013268e-20,
    -4.8622389031500010636e-20,
    -4.8003746093201616167e-20,
    -4.7396835396056984764e-20,
    -4.6801362142501976304e-20,
    -4.621704073644220051e-20,
    -4.564359444074631573e-20,
    -4.5080755049521610664e-20,
    -4.4528262574447228118e-20,
    -4.3985864944480063777e-20,
    -4.345331771828572485e-20,
    -4.293038380878199908e-20,
    -4.2416833219215253855e-20,
    -4.1912442790221187573e-20,
    -4.141699595735057459e-20,
    -4.0930282518568055114e-20,
    -4.045209841125794141e-20,
    -3.9982245498295321748e-20,
    -3.9520531362763728372e-20,
    -3.906676911092226449e-20,
    -3.8620777183045467817e-20,
    -3.8182379171778464848e-20,
    -3.775140364766809272e-20,
    -3.7327683991547819605e-20,
    -3.6911058233470486198e-20,
    -3.650136889789813957e-20,
    -3.6098462854872718676e-20,
    -3.5702191176904958385e-20,
    -3.5312409001331782045e-20,
    -3.4928975397904704835e-20,
    -3.4551753241383235295e-20,
    -3.418060908891824813e-20,
    -3.38154130620206231e-20,
    -3.345603873292020081e-20,
    -3.3102363015129452238e-20,
    -3.275426605803495085e-20,
    -3.2411631145348158784e-20,
    -3.2074344597254843682e-20,
    -3.1742295676109981407e-20,
    -3.1415376495532095648e-20,
    -3.109348193275767598e-20,
    -3.077650954412276709e-20,
    -3.0464359483544849638e-20,
    -3.015693442388389778e-20,
    -2.9854139481066990008e-20,
    -2.9555882140866035265e-20,
    -2.9262072188223106534e-20,
    -2.8972621639022595788e-20,
    -2.868744467421384236e-20,
    -2.8406457576192158342e-20,
    -2.8129578667350183195e-20,
    -2.7856728250715377933e-20,
    -2.7587828552593096454e-20,
    -2.7322803667138176764e-20,
    -2.7061579502781289532e-20,
    -2.6804083730439465565e-20,
    -2.6550245733443195141e-20,
    -2.629999655911540948e-20,
    -2.6053268871940326104e-20,
    -2.5809996908262803759e-20,
    -2.557011643246129144e-20,
    -2.5333564694539842597e-20,
    -2.5100280389086943953e-20,
    -2.487020361555100907e-20,
    -2.4643275839784521724e-20,
    -2.4419439856810700504e-20,
    -2.4198639754768494211e-20,
    -2.3980820879993465798e-20,
    -2.3765929803193867166e-20,
    -2.355391428668279075e-20,
    -2.334472325262893737e-20,
    -2.3138306752289902179e-20,
    -2.2934615936193449297e-20,
    -2.2733603025233490223e-20,
    -2.2535221282648863289e-20,
    -2.2339424986854232003e-20,
    -2.2146169405093597539e-20,
    -2.1955410767888115143e-20,
    -2.1767106244250929472e-20,
    -2.1581213917642848606e-20,
    -2.1397692762643659162e-20,
    -2.121650262231482129e-20,
    -2.103760418623025554e-20,
    -2.0860958969152752875e-20,
    -2.0686529290334432897e-20,
    -2.0514278253420467801e-20,
    -2.0344169726936032748e-20,
    -2.0176168325337268947e-20,
    -2.0010239390607649911e-20,
    -1.9846348974381923932e-20,
    -1.9684463820580401342e-20,
    -1.9524551348537001605e-20,
    -1.9366579636605089862e-20,
    -1.9210517406225693772e-20,
    -1.905633400644325858e-20,
    -1.8903999398854637624e-20,
    -1.8753484142977501433e-20,
    -1.8604759382024884684e-20,
    -1.8457796829073024429e-20,
    -1.831256875361010383e-20,
    -1.8169047968453987946e-20,
    -1.8027207817027396233e-20,
    -1.788702216097941352e-20,
    -1.7748465368142612957e-20,
    -1.7611512300815405308e-20,
    -1.747613830435964991e-20,
    -1.7342319196103842303e-20,
    -1.721003125454257065e-20,
    -1.7079251208823231713e-20,
    -1.694995622851130363e-20,
    -1.6822123913625781254e-20,
    -1.6695732284936642869e-20,
    -1.6570759774516514479e-20,
    -1.6447185216538941852e-20,
    -1.6324987838315938223e-20,
    -1.6204147251567741435e-20,
    -1.6084643443917900453e-20,
    -1.5966456770607096573e-20,
    -1.5849567946419281606e-20,
    -1.5733958037813930878e-20,
    -1.5619608455258439183e-20,
    -1.5506500945754843392e-20,
    -1.5394617585555272932e-20,
    -1.5283940773060694385e-20,
    -1.5174453221897715826e-20,
    -1.5066137954168329797e-20,
    -1.4958978293867725739e-20,
    -1.4852957860465355624e-20,
    -1.4748060562644681764e-20,
    -1.4644270592197115654e-20,
    -1.4541572418065828277e-20,
    -1.4439950780535252155e-20,
    -1.4339390685562199194e-20,
    -1.4239877399244682197e-20,
    -1.4141396442424613875e-20,
    -1.4043933585420702468e-20,
    -1.3947474842887968835e-20,
    -1.3852006468800404283e-20,
    -1.3757514951553430623e-20,
    -1.366398700918288266e-20,
    -1.3571409584697371929e-20,
    -1.3479769841520959101e-20,
    -1.3389055159043170024e-20,
    -1.32992531282734776516e-20,
    -1.32103515475974494266e-20,
    -1.3122338418631857318e-20,
    -1.30352019421761196485e-20,
    -1.29489305142575261884e-20,
    -1.28635127222677641326e-20,
    -1.27789373411883577184e-20,
    -1.2695193329902679745e-20,
    -1.26122698275922734715e-20,
    -1.2530156150215296598e-20,
    -1.2448841787064950299e-20,
    -1.23683163974058210185e-20,
    -1.22885698071861332556e-20,
    -1.22095920058239578734e-20,
    -1.2131373143065476118e-20,
    -1.2053903525913468578e-20,
    -1.1977173615624235142e-20,
    -1.1901174024771203295e-20,
    -1.1825895514373553841e-20,
    -1.1751328991088208251e-20,
    -1.16774655044635890225e-20,
    -1.1604296244253612658e-20,
    -1.15318125377903985994e-20,
    -1.1460005847414239123e-20,
    -1.138886776795940641e-20,
    -1.131839002429441519e-20,
    -1.1248564468915395547e-20,
    -1.1179383079591270814e-20,
    -1.1110837957059470401e-20,
    -1.1042921322770939497e-20,
    -1.0975625516683246217e-20,
    -1.09089429951006202885e-20,
    -1.0842866328559773705e-20,
    -1.0777388199760419531e-20,
    -1.0712501401539384779e-20,
    -1.064819883488729786e-20,
    -1.05844735070068095995e-20,
    -1.0521318529411371332e-20,
    -1.04587271160635987576e-20,
    -1.0396692581552276922e-20,
    -1.03352083393071067564e-20,
    -1.02742678998502915586e-20,
    -1.0213864869084093624e-20,
    -1.0153992946613538407e-20,
    -1.00946459241034119285e-20,
    -1.0035817683668786926e-20,
    -9.977502196298270812e-21,
    -9.919693520309233679e-21,
    -9.862385799834270233e-21,
    -9.805573263338173728e-21,
    -9.749250222164732452e-21,
    -9.6934110691126502065e-21,
    -9.6380502770399369e-21,
    -9.5831623974961163886e-21,
    -9.528742059381617345e-21,
    -9.47478396763373688e-21,
    -9.421282901938574955e-21,
    -9.368233715468347519e-21,
    -9.3156313336435258164e-21,
    -9.263470752919218914e-21,
    -9.211747039595287624e-21,
    -9.160455328649642647e-21,
    -9.1095908225942109775e-21,
    -9.059148790353082744e-21,
    -9.009124566162334732e-21,
    -8.9595135484910595096e-21,
    -8.910311198983132382e-21,
    -8.8615130414192659845e-21,
    -8.8131146606989006736e-21,
    -8.765111701841508216e-21,
    -8.7174998690068803136e-21,
    -8.670274924533993949e-21,
    -8.623432687998049683e-21,
    -8.576969035285295574e-21,
    -8.53087989768524712e-21,
    -8.485161260999936165e-21,
    -8.439809164669827073e-21,
    -8.3948197009160341586e-21,
    -8.350189013898505974e-21,
    -8.3059132988898282136e-21,
    -8.261988801464321808e-21,
    -8.218411816702108856e-21,
    -8.17517868840783412e-21,
    -8.132285808343732739e-21,
    -8.089729615476746148e-21,
    -8.047506595239387813e-21,
    -8.005613278804081045e-21,
    -7.9640462423706812495e-21,
    -7.922802106466911705e-21,
    -7.881877535261450244e-21,
    -7.841269235889396545e-21,
    -7.800973957789879136e-21,
    -7.760988492055540298e-21,
    -7.7213096707936610455e-21,
    -7.681934366498696451e-21,
    -7.642859491435968189e-21,
    -7.604081997036317211e-21,
    -7.565598873301466778e-21,
    -7.527407148219896656e-21,
    -7.4895038871930130455e-21,
    -7.451886192471402862e-21,
    -7.414551202600974695e-21,
    -7.377496091878789765e-21,
    2.2332872434632680135e-20,
];

pub const BS: [f64; TEST_DATA_LEN] = [
    0.001,
    0.0007066839714471122602,
    0.0005457089552238805941,
    0.0004438073394495412823,
    0.00037378263937896965256,
    0.00032277126846663270373,
    0.00028397890035422762867,
    0.000253494396915290998,
    0.00022891143286039416905,
    0.00020866971564923443944,
    0.00019171376329787233977,
    0.0001773042644404332124,
    0.0001649080512683836885,
    0.00015413098819717657909,
    0.00014467545638945233216,
    0.00013631253055238715945,
    0.00012886325071427535459,
    0.00012218570147973964872,
    0.000116165901961702326376,
    0.00011071125715497137982,
    0.000105745768433235279713,
    0.000101206475663385358056,
    9.704077673042409709e-05,
    9.320438206093803469e-05,
    8.965973534971644582e-05,
    8.6374781085814360547e-05,
    8.332199318407724685e-05,
    8.047760239848331187e-05,
    7.782097663145002324e-05,
    7.5334119970139908794e-05,
    7.300126473643795995e-05,
    7.080853700983929404e-05,
    6.8743680636871910156e-05,
    6.6795828134835526215e-05,
    6.4955309450893810886e-05,
    6.3213491475166790025e-05,
    6.156264268905201774e-05,
    5.9995818473257924162e-05,
    5.850676348810558972e-05,
    5.7089828233295893353e-05,
    5.5739897441250538226e-05,
    5.445232839141764184e-05,
    5.3222897578223016195e-05,
    5.2047754442062424962e-05,
    5.0923381095673144673e-05,
    4.984655715875872532e-05,
    4.8814328960645812222e-05,
    4.7823982490845574195e-05,
    4.6873019576003276904e-05,
    4.595913684303428783e-05,
    4.5080207095569512793e-05,
    4.423426278679204575e-05,
    4.3419481318432343686e-05,
    4.2634171934778026463e-05,
    4.187676401339553381e-05,
    4.1145796581944424602e-05,
    4.0439908913875643752e-05,
    3.9757832075662992453e-05,
    3.909838131511145959e-05,
    3.8460449194700290523e-05,
    3.78429993862508266e-05,
    3.7245061053788035197e-05,
    3.6665723760562970742e-05,
    3.6104132844047833107e-05,
    3.555948520949493146e-05,
    3.503102549852393053e-05,
    3.451804259430096326e-05,
    3.4019866429310192212e-05,
    3.3535865065587410395e-05,
    3.3065442020665901185e-05,
    3.2608033815444621804e-05,
    3.216310772278519475e-05,
    3.1730159697926123372e-05,
    3.130871247381184851e-05,
    3.089831380620645979e-05,
    3.0498534855027760593e-05,
    3.010896868972326509e-05,
    2.9729228907738315236e-05,
    2.9358948356217407511e-05,
    2.8997777948049957713e-05,
    2.8645385564235786564e-05,
    2.8301455035316294656e-05,
    2.7965685195305721085e-05,
    2.7637789002172653857e-05,
    2.7317492719473516708e-05,
    2.7004535154234470113e-05,
    2.6698666946622424214e-05,
    2.6399649907345372832e-05,
    2.610725639908193506e-05,
    2.5821268758564216734e-05,
    2.554147875623071439e-05,
    2.5267687090630371157e-05,
    2.4999702914998039126e-05,
    2.4737343393638173961e-05,
    2.4480433285949905548e-05,
    2.4228804556104802051e-05,
    2.3982296006550487035e-05,
    2.3740752933660462215e-05,
    2.3504026803984454903e-05,
    2.3271974949675687711e-05,
    2.3044460281782796965e-05,
    2.2821351020195760204e-05,
    2.2602520439128054156e-05,
    2.23878466271021904e-05,
    2.2177212260483509029e-05,
    2.1970504389678329409e-05,
    2.1767614237177842552e-05,
    2.1568437006689065858e-05,
    2.1372871702649188537e-05,
    2.1180820959470238994e-05,
    2.099219087990751187e-05,
    2.080689088198803903e-05,
    2.0624833553974827461e-05,
    2.0445934516878989325e-05,
    2.0270112294065424709e-05,
    2.0097288187528726665e-05,
    1.992738616044459571e-05,
    1.9760332725628520432e-05,
    1.9596056839557982074e-05,
    1.9434489801637093259e-05,
    1.9275565158403606393e-05,
    1.9119218612397679935e-05,
    1.8965387935429862756e-05,
    1.8814012886002515694e-05,
    1.8665035130654456425e-05,
    1.8518398169013089458e-05,
    1.8374047262351724493e-05,
    1.8231929365462313642e-05,
    1.8091993061665486027e-05,
    1.7954188500790608754e-05,
    1.7818467339968723332e-05,
    1.7684782687090635334e-05,
    1.7553089046791236581e-05,
    1.742334226882935173e-05,
    1.7295499498740076338e-05,
    1.7169519130643740023e-05,
    1.7045360762102331111e-05,
    1.6922985150920490197e-05,
    1.6802354173794047154e-05,
    1.6683430786714568435e-05,
    1.6566178987043527409e-05,
    1.6450563777174536455e-05,
    1.6336551129706598236e-05,
    1.6224107954055577524e-05,
    1.6113202064435073962e-05,
    1.6003802149141618507e-05,
    1.5895877741082618826e-05,
    1.5789399189488790555e-05,
    1.5684337632755896238e-05,
    1.5580664972363552774e-05,
    1.5478353847821587848e-05,
    1.5377377612597033298e-05,
    1.527771031097725686e-05,
    1.5179326655827043207e-05,
    1.5082202007199574802e-05,
    1.4986312351763323659e-05,
    1.489163428300876116e-05,
    1.4798144982200624831e-05,
    1.47058222000431802556e-05,
    1.4614644239027534576e-05,
    1.4524589936431581766e-05,
    1.4435638647944606176e-05,
    1.43477702318899255526e-05,
    1.42609650340202483074e-05,
    1.4175203872861637282e-05,
    1.4090468025583124319e-05,
    1.4006739214370109316e-05,
    1.3923999593280716948e-05,
    1.3842231735565253676e-05,
    1.376141862142984446e-05,
    1.36815436262261991174e-05,
    1.3602590509050296651e-05,
    1.3524543401733564862e-05,
    1.34473867982108790384e-05,
    1.3371105544250420522e-05,
    1.3295684827531105399e-05,
    1.3221110168053937331e-05,
    1.3147367408874249318e-05,
    1.3074442707142371618e-05,
    1.3002322525440821278e-05,
    1.2930993623406624425e-05,
    1.28604430496278795085e-05,
    1.2790658133804151456e-05,
    1.272162647916072253e-05,
    1.2653335955107164892e-05,
    1.2585774690131104262e-05,
    1.2518931064918425715e-05,
    1.2452793705691555733e-05,
    1.23873514777577894715e-05,
    1.23225934792599857706e-05,
    1.2258509035122255901e-05,
    1.21950876911835872064e-05,
    1.2132319208512633141e-05,
    1.20701935578971701455e-05,
    1.2008700914501997821e-05,
    1.1947831652689303918e-05,
    1.1887576340995757354e-05,
    1.1827925737260826522e-05,
    1.1768870783901037484e-05,
    1.1710402603325094666e-05,
    1.1652512493484992517e-05,
    1.1595191923558433077e-05,
    1.1538432529758052906e-05,
    1.1482226111263131951e-05,
    1.1426564626269631677e-05,
    1.1371440188154563975e-05,
    1.1316845061750850519e-05,
    1.1262771659728974861e-05,
    1.1209212539081875476e-05,
    1.1156160397709654958e-05,
    1.11036080711008199685e-05,
    1.1051548529106877376e-05,
    1.0999974872807242639e-05,
    1.0948880331461520515e-05,
    1.0898258259546330463e-05,
    1.0848102133873957254e-05,
    1.079840555079019532e-05,
    1.0749162223448867565e-05,
    1.0700365979160574539e-05,
    1.0652010756813331361e-05,
    1.060409060436282511e-05,
    1.0556599676390113813e-05,
    1.0509532231724654849e-05,
    1.0462882631130645545e-05,
    1.0416645335054701662e-05,
    1.0370814901432999764e-05,
    1.0325385983556049013e-05,
    1.0280353327989338699e-05,
    1.0235711772548162766e-05,
    1.0191456244324977473e-05,
    1.0147581757767716523e-05,
    1.0104083412807526632e-05,
    1.0060956393034451871e-05,
    1.0018195963919640731e-05,
    9.975797471082692669e-06,
    9.933756338602819718e-06,
    9.892068067372529089e-06,
    9.850728233492587254e-06,
    9.809732486707063561e-06,
    9.769076548877283584e-06,
    9.728756212493580272e-06,
    9.688767339223742083e-06,
    9.649105858497116397e-06,
    9.6097677661233464076e-06,
    9.570749122944756872e-06,
    9.532046053521438494e-06,
    9.493654744848107056e-06,
    9.4555714451018455635e-06,
    9.417792462419867566e-06,
    9.380314163706460655e-06,
    9.343132973468306468e-06,
    9.306245372677389544e-06,
    9.269647897660733553e-06,
    9.233337139016236235e-06,
    9.197309740553881939e-06,
    9.161562398261646347e-06,
    9.126091859295425403e-06,
    9.090894920992333037e-06,
    9.055968429906748144e-06,
    9.021309280868493975e-06,
    8.986914416062563345e-06,
    8.952780824129815882e-06,
    8.9189055392880907245e-06,
    8.885285640473198339e-06,
    8.851918250499265806e-06,
    8.8188005352379317826e-06,
    8.785929702815897464e-06,
    8.7533030028303554955e-06,
    8.720917725581838756e-06,
    8.688771201324031531e-06,
    8.656860799530114484e-06,
    8.625183928175216628e-06,
    8.59373803303456123e-06,
    8.562520596996911997e-06,
    8.53152913939292675e-06,
    8.500761215338044874e-06,
    8.470214415089541195e-06,
    8.439886363417396134e-06,
    8.4097747189886268175e-06,
    8.379877173764758286e-06,
    8.350191452412094654e-06,
    8.3207153117244852994e-06,
    8.2914465400582716564e-06,
    8.262382956779117328e-06,
    8.233522411720433709e-06,
    8.204862784653113616e-06,
    8.1764019847663037545e-06,
    8.148137950158944146e-06,
    8.120068647341817661e-06,
    8.09219207074985764e-06,
    8.064506242264463921e-06,
    8.037009210745594586e-06,
    8.009699051573394581e-06,
    7.982573866199139936e-06,
    7.955631781705274565e-06,
    7.928870950374326787e-06,
    7.902289549266497845e-06,
    7.875885779805717462e-06,
    7.849657867373971014e-06,
    7.823604060913705451e-06,
    7.797722632538127413e-06,
    7.772011877149209034e-06,
    7.7464701120632282965e-06,
    7.721095676643667871e-06,
    7.695886931941304364e-06,
    7.670842260341326417e-06,
    7.645960065217320449e-06,
    7.621238770591967761e-06,
    7.59667682080430393e-06,
    7.5722726801833917367e-06,
    7.5480248327282623926e-06,
    7.5239317817939880473e-06,
    7.4999920497837472153e-06,
    7.476204177846748247e-06,
    7.452566725581886319e-06,
    7.429078270747002458e-06,
    7.4057374089736227817e-06,
    7.3825427534870602744e-06,
    7.3594929348317582034e-06,
    7.3365866006017633308e-06,
    7.3138224151762168627e-06,
    7.2911990594597546808e-06,
    7.2687152306277103555e-06,
    7.2463696418760180672e-06,
    7.2241610221757147334e-06,
    7.2020881160319421895e-06,
    7.1801496832473541802e-06,
    7.1583444986898355014e-06,
    7.136671352064437822e-06,
    7.115129047689450299e-06,
    7.093716404276508831e-06,
    7.0724322547146680603e-06,
    7.051275445858345958e-06,
    7.030244838319065015e-06,
    7.009339306260908975e-06,
    6.98855773719961616e-06,
    6.9678990318052389843e-06,
    6.9473621037082922627e-06,
    6.926945879309317361e-06,
    6.9066492975917991427e-06,
    6.8864713099383561396e-06,
    6.8664108799501490138e-06,
    6.846466983269432503e-06,
    6.826638607405192766e-06,
    6.8069247515618053407e-06,
    6.7873244264706520465e-06,
    6.767836654224641365e-06,
    6.748460468115568826e-06,
    6.7291949124742645955e-06,
    6.710039042513471237e-06,
    6.690991924173396572e-06,
    6.672052633969890528e-06,
    6.6532202588451935306e-06,
    6.6344938960212032456e-06,
    6.6158726528552163633e-06,
    6.597355646698087413e-06,
    6.578942004754767873e-06,
    6.560630863947172617e-06,
    6.542421370779328079e-06,
    6.5243126812047630432e-06,
    6.5063039604960937586e-06,
    6.4883943831167634034e-06,
    6.4705831325948937834e-06,
    6.4528694013992102813e-06,
    6.4352523908169973083e-06,
    6.417731310834049061e-06,
    6.400305380016575249e-06,
    6.382973825395025953e-06,
    6.365735882349798463e-06,
    6.3485907944987921967e-06,
    6.331537813586774269e-06,
    6.3145761993765239044e-06,
    6.2977052195417233225e-06,
    6.2809241495615585654e-06,
    6.264232272617003024e-06,
    6.247628879488748864e-06,
    6.231113268456758456e-06,
    6.2146847452014041723e-06,
    6.198342622706168322e-06,
    6.182086221161873899e-06,
    6.1659148678724189945e-06,
    6.1498278971619854688e-06,
    6.1338246502836982832e-06,
    6.1179044753297061616e-06,
    6.1020667271426586568e-06,
    6.0863107672285559943e-06,
    6.0706359636709432663e-06,
    6.055041691046430029e-06,
    6.0395273303415065597e-06,
    6.0240922688706355306e-06,
    6.0087359001955993386e-06,
    5.993457624046072588e-06,
    5.9782568462414094052e-06,
    5.9631329786136131846e-06,
    5.948085438931474557e-06,
    5.9331136508258555303e-06,
    5.918217043716097898e-06,
    5.9033950527375379157e-06,
    5.888647118670108313e-06,
    -5.9657899234079681765e-06,
];

#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 0.00001;

    #[test]
    fn formula_test() {
        let steps_to_convert = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000];
        let steps_from_tge = [
            1,
            10,
            100,
            1000,
            10000,
            100000,
            1000000,
            10000000,
            100000000,
            1000000000,
            10000000000,
            100000000000,
            1000000000000,
            10000000000000,
            100000000000000,
            1000000000000000u64,
        ];
        let mut test_number = 0;
        for tge in steps_from_tge {
            for steps in steps_to_convert {
                let formula_res = formula(tge as f64, steps as f64) as f64 / 1e+18;
                let diff = formula_res - TEST_RESULTS[test_number];
                assert!(diff.abs() < EPS);
                test_number += 1;
            }
        }
    }

    pub const TEST_RESULTS: [f64; 144] = [
        0.0009999999999993069988,
        0.0099999999999722827965,
        0.09999999999764401515,
        0.99999999976855979966,
        9.999999976897553822,
        99.99999769017119888,
        999.9997690212771886,
        9999.97690216929368,
        99997.69021734510898,
        0.0009999999999951495171,
        0.009999999999930706679,
        0.09999999999722826438,
        0.99999999976440212546,
        9.99999997685597819,
        99.99999768975543191,
        999.99976901711954724,
        9999.9769021277170395,
        99997.69021692934621,
        0.0009999999999535733996,
        0.0099999999995149455034,
        0.09999999999307064569,
        0.99999999972282604954,
        9.9999999764402165425,
        99.999997685597818986,
        999.9997689755434749,
        9999.9769017119560885,
        99997.69021277173306,
        0.0009999999995378124409,
        0.009999999995357337218,
        0.09999999995149456977,
        0.99999999930706517937,
        9.999999972282608951,
        99.99999764402173241,
        999.99976855978263757,
        9999.9768975543483975,
        99997.690171195645235,
        0.0009999999953802037218,
        0.0099999999537812491596,
        0.099999999535733699596,
        0.9999999951494564776,
        9.9999999307065223775,
        99.99999722826086668,
        999.99976440217392337,
        9999.9768559782605735,
        99997.689755434781546,
        0.0009999999538041167476,
        0.009999999538020380718,
        0.099999995378124997836,
        0.999999953573369571,
        9.999999514945651313,
        99.999993070652180904,
        999.9997228260870088,
        9999.976440217391428,
        99997.68559782608645,
        0.0009999995380432472221,
        0.009999995380411684162,
        0.09999995380203804962,
        0.9999995378124999501,
        9.999995357336956658,
        99.9999514945652237,
        999.9993070652174083,
        9999.972282608696332,
        99997.644021739135496,
        0.0009999953804345515333,
        0.009999953804324729009,
        0.099999538041168484215,
        0.99999538020380429604,
        9.999953781249999452,
        99.99953573369565163,
        999.99514945652174447,
        9999.930706521739921,
        99997.22826086956775,
        0.0009999538043475950794,
        0.009999538043455163602,
        0.099995380432472830146,
        0.99995380411684786637,
        9.999538020380434489,
        99.995378125000002,
        999.95357336956521976,
        9999.514945652173992,
        99993.07065217391937,
        0.0009995380434780298893,
        0.009995380434759511268,
        0.09995380434551630333,
        0.99953804324728257047,
        9.995380411684783084,
        99.953802038043477296,
        999.5378124999999727,
        9995.357336956521976,
        99951.49456521739194,
        0.0009953804347823777716,
        0.009953804347802989658,
        0.0995380434759510907,
        0.99538043455163038864,
        9.9538043247282601556,
        99.53804116847825867,
        995.3802038043478433,
        9953.78125,
        99535.73369565217581,
        0.0009538043478258559447,
        0.009538043478237771822,
        0.09538043478029890887,
        0.95380434759510868137,
        9.538043455163043305,
        95.38043247282608661,
        953.80411684782609427,
        9538.020380434782055,
        95378.125,
        0.0005380434782607852349,
        0.005380434782600264018,
        0.053804347825243750703,
        0.53804347817654929464,
        5.3804347741766713753,
        53.804346982884489137,
        538.04339394062299107,
        5380.4263505840362996,
        53803.50462362102553,
        0.00010531914893616588951,
        0.0010531914893612702357,
        0.0105319148935738241685,
        0.10531914893185048698,
        1.053191488929729136,
        10.531914850419713758,
        105.31914461643948755,
        1053.19105738863027,
        10531.871696309839535,
        1.16470588235293541626e-05,
        0.00011647058823528841538,
        0.0011647058823523713262,
        0.011647058823472423994,
        0.11647058822959541202,
        1.1647058817830711064,
        11.647058766542404129,
        116.47058253659338334,
        1164.705312482868294,
        1.1895246692412392412e-06,
        1.1891116398421971423e-05,
        0.00011890600111673168301,
        0.0011890610437408145102,
        0.011890606307114153298,
        0.118906064103715028435,
        1.1890606420697238388,
        11.890606361840548644,
        118.906058271739922816,
    ];
}
