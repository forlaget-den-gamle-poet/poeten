pub const TEMA_OPLÆSNING: u64 = 1u64 << 33;

/// First line of a poem is also the title
pub fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

/// Everything after the first line (the title) is the poem body
pub fn body(s: &str) -> &str {
    match s.find('\n') {
        Some(pos) => &s[pos + 1..],
        None => "",
    }
}

pub static SAMLINGER: [&str; 31] = [
    "Aftryk",
    "Billedbogen",
    "Brønden",
    "Dagen",
    "Landskab",
    "Det ender med et smil",
    "Klodeskyer",
    "Filibuster",
    "Firs tekster",
    "Grib",
    "Halvfjerdserbrun",
    "Hjertets rytme",
    "Hjertets slag",
    "Jordens liv",
    "Konserves",
    "Kun et øjeblik",
    "Lys i mørket",
    "Månelys",
    "Natur",
    "Opdagelsesrejse",
    "Overvejelser",
    "Perler på snor",
    "Skygger",
    "Stengrunds tørst",
    "Stilhedens styrke",
    "Tegn",
    "Til Dig",
    "Undervejs",
    "Vækst",
    "Vejen",
    "Yrk",
];

pub static TEMAER: [&str; 40] = [
    "12 mest læste",
    "Aforismer",
    "Aldring",
    "Barnet",
    "Blomster",
    "Døden",
    "Døren",
    "Dagen",
    "De første år",
    "De indre dæmoner",
    "De ondskabsfulde",
    "De politisk inspirerede",
    "Demens",
    "Eksistens",
    "Erindringer",
    "Følelser",
    "Familien",
    "Farver",
    "Fra årets gang",
    "Fuglen",
    "Håb og Glæde",
    "Havet",
    "Hindsholm",
    "Huset",
    "Identitet",
    "Isolation",
    "Kærlighed",
    "Livskraften",
    "Lyset - Mørket",
    "Løb",
    "Magtspil",
    "Morgen",
    "Natten",
    "Oplæsning",
    "Portræt",
    "Regnbuen",
    "Samliv",
    "Sorg",
    "Træet",
    "Tro",
];

pub static DIGTE: [(&str, u64, &str);1727]=[
("Aftryk",283467890724,"Aftryk

Tapetet husker dig
som et motiv på en 
trykklods dyppet i
farve og udleveret til
en eftertid der 
ikke ved af dig før 
du konfronteres med dit
aftryk
Da de fældede det gamle
kastanjetræ i
haven døde Amanda i sin
silkekjole under den 
brede stråhat og
det kys der blev 
givet som et
pant på hjertets længsel efter
enhed forsvandt i 
lyden af motorsav
Husker du duften af 
markskellenes syrener eller 
er det syrenerne der 
husker dig
                  "),
("Aftryk",184590904,"Barn

Et  styrtende sekund
griber jeg
livets skrøbelighed,
magnoliablomstens
sårede blade efter
forårsnattens frost


Oktober 2010"),
("Aftryk",558479966240,"Buketter

Så rødt er altså blod,
så rødt som kærlighed
spildt i sne,
et skrig i
mørket en vinternat;
så rødt er blod som
hadet der ætser
folkeslag og 
sætter kulturer i 
stå,
så rødt som  valmuens
vuggende bæger en
sommerdagstid og vi
går forbi og vender vi
os ser vi vore spor i
sneen som en stemplet
påstand der forsvinder 
bagud og bliver væk i
ingenting og 
betydningen ligger
forude eller måske
lige her hvor sneen
råber afmægtigt og
tomheden  splintres til
ufattelighed
Så rød er sneen 
hvor én er død og
en anden har høstet for at 
fortsætte fremad det
stykke vej der er til
enden hvor svaret vel
venter, at ingen forgæves 
betræder den 
vej der 
former sig under
vore skridt og hvor 
sporet ender
finder vi hænderne tunge af
plukkede vækster vi
rækker frem og var vi 
alene engang, er vi det
ikke her



Januar 2010




                  "),
("Aftryk",628407412736,"Bøn

Lad guderne
forsone sig eller
føre krig mod
hinanden i evigheden
som de vil og lad os
mennesker have fred med
hinanden til at
udfolde livet den 
korte tid vi har
til rådighed

Amen


April 2010"),
("Aftryk",167780356,"De sidste år

Til sidst bliver det
svært og kun et
langt livs opsparet
visdom kan kaste et
forsonende lys over
svækkelsens åbenbare
udfordring


Juni 2010

                  "),
("Aftryk",17498636800,"Fortabelse

Jeg følger
mørket for ikke at
udfordre lyset der
knivskarpt kaster min
skygge fra sig 

Jeg går alene for ikke at
møde sorgen bag  den
anden

Jeg sætter ingen
spor for ikke at 
møde mig selv 


23. nov. 2010

                  "),
("Aftryk",1226352640,"Frihed

Sollyset lader sig aldrig
tæmme men 
kaster fængslets tremmer
mod cellens vægge

Fuglen synger sig
gennem sit
bur og fanger livets 
triumferende tone



8. februar 2010"),
("Aftryk",1536,"Graffiti

Han var her i
dag og 
overmalede
gavlen med sine
bekymringer
Langs plankeværket
tegnede han
små spidse
frustrationer under en
vild påstand

Inden han gik
såede han en
håndfulde mistanke der
med tiden kan blive
besværligt ukrudt i
bedet foran
facaden


Juni 2010
                  "),
("Aftryk",3584,"Hvad så

om nu de blev de
eneste af
alle jordens ord

fundet af
forskere der lod dem
studere

anbragt mellem
fragmenter af
vraggods

ophøjet som
gravskrift over
en civilisation der
formåede at

frembringe dette

Ja, hvad så?



April 2010"),
("Aftryk",688520274444,"Kradsebræt

Jeg vil ikke længere være dit
kradsebræt,
ikke længere hænge udspændt
mellem pligt og kærlighed til
et barn der for længst har
nået den modne alder
Jeg vil ikke dømmes  af dig for
mit liv, jeg har levet som
det blev
Kun den der kan se
mit hjerte skal være min
dommer og med al min
kærlighed levende ved jeg at
det hverken er dig eller
nogen anden jeg kender


12. januar 2011"),
("Aftryk",549891088416,"Livets grænse

Hvorfor binde tanker til et
liv efter døden når
livet står på spring og
kræver at blive
levet nu
Det er nu du skal 
leve
Siden skal du dø for
sådan slutter  ethvert liv
som vi kender det og 
tilstanden derefter 
giver kun mening for den der
genfinder sig selv
bag grænsen



April 2010"),
("Aftryk",17179902464,"Lommeliv

Jeg gør min verden
lillebitte

krammer den
sammen til en
hård kugle og
pakker den ind i
bekymring med
skarpe kanter

De stikker gennem 
lommen når jeg 
går"),
("Aftryk",8742764544,"Lærke

Hjertesangen over den
spirende sæd,
være i
dit hjerte -
være hvor dit
hjerte er -
i dit hjerte -
hvor dit hjerte er -
hjertelydens kald
over den
skabende stilhed
ved om en vej
 

Maj 2010

                  "),
("Aftryk",8589934592,"Mellemrum

Du er muligvis og
muligvis ikke
Måske
måske ikke
Du er eventuelt og 
smiler et
sted mellem
uudgrundelighed og
fjernt
Dine øjne tager ikke
stilling og
disse hænder er
tomme for
retning og
hænger fra
arme der mangler
et ord
et blik
en bevægelse
for at finde sig selv



April 2010"),
("Aftryk",214865838592,"Mistanken

Som et stjerneskud der
flænser himlen i en
glødende stribe
styrter tanken ind i 
hendes sind og 
slår ned

Ud af Helvede
dukker øksen frem og
kløver hendes
verden 

Hun gemmer sine
øjne mellem
bladene og
går frem ad stien
ved hans side
Fuglene synger om
blomstrende roser
på den anden side
hvor han ler mod
den åbne himmel mens det 
glødende ørkensand
brænder hendes fødder

Hun ved ikke 
længere hvem
hun er eller hvor
hun går hen

Hun følges med en
ukendt
forklædt som 
ham

                  "),
("Aftryk",232045707776,"Og hvem er jeg

Husker du de
øjne du brændte den 
lange vinter du
lod lukke dit bryst,
den smerte du
lod visne livet,  et
forår der aldrig
sprang i 
blomst
Du husker vel den
angst du gemte bag
din skygge, måske en 
flig af indsigt i
dit eget svigt

Måske  du tør
bøje dig og
plukke din
skæbne


Marts 2010"),
("Aftryk",33288,"Pædagogik

Plagsomme tanker er som
plagsomme børn der bør
ignoreres indtil de 
finder en rimelig 
adfærd

Juni 2010
                  "),
("Aftryk",94590042624,"Samliv
 
Væggene er kalket
tomme
Gulvet belagt med
ørken og
intet vindue
får lov at se
Sporene er 
parallelle men
de opdager aldrig 
hinanden og
ingen finder på at 
hviske et navn

De går på vejen
som må være her 
i den stumme
lydighed mod det
fælles løfte de
afgav engang
for kærlighedens
ansigt

Juni 2010
                  "),
("Aftryk",25803391488,"Skuffelse

Næsen kan blive så
lang at man har 
svært ved at
holde den fri af
jorden
Synsfeltet indskrænkes til en
snæver spalte og 
kun ved at
skule  kan man 
snappe et glimt af 
horisonten
Det værste er det
svineri man går og
roder op og den
hudløse smerte


November 2010"),
("Aftryk",8740937728,"Skæbnekast

Vi er som kast fra
mørket gennem
eksistensen

I bevægelse
udfoldes skæbnen
båret af 
livets vilje

Lysets skaberkraft
farer gennem os
det øjeblik det
tager at føle
uendelighedens
styrke


maj 2010"),
("Aftryk",3072,"Sommertalen

Ords sommerfugleflugt fra
bløde læber
Hænders  svimle arabesker
Øjnes nødblus i
det blå

Våde klask af
faldende argumenter



Juni 2010"),
("Aftryk",81674665984,"Stjerneskrift

Sporene under dem og 
universet over,
lyden af havet og
mørket der ikke
skjuler men
åbner det øjeblik da
hænder griber 
hænder som var
de skibsbrudne der
kæmper for
frelse men endnu ikke
tør tro at livet er
givet tilbage og
blotter  et 
levende nu

Alt er skrevet i
sandet under lyset af
stjernernes vandring
den lange nat
før dag


Juni 2010
                  "),
("Aftryk",30098375172,"Tapet

Dagen var næsten
slut  da hun
iførte sig nattens 
tornekrone og
talte sine år 
Så mange og
alligevel for få til at
rette det, der
gik skævt
Hvem har jeg
talt med i dag
spurgte hun væggen og 
huskede kassedamen 
der måske havde hørt
hendes stemme da hun
sagde tak og
siden -
i lige måde
Så sagde hun tak igen og
så gennem tapetet sin 
barndoms landskab
befolket med dem der
altid var ved 
hendes side
Hvad andet var der vel
at sige



10. februar 2010"),
("Aftryk",8642371584,"Tidevand

Himlen står på søjler af
lys over
fjordens sølv
Uden hensigt tegner
sandet mit spor på
sin nøgne flade
Om lidt er
alting rent og 
selv jeg har glemt 
hvem jeg var og 
hvor jeg ville hen med 
det jeg bar


19. marts 2011

                  "),
("Aftryk",558347059200,"Træk

Himlen driver tegn

Gåsetrækkets kile kalder
længslens navn, strandet mellem
jord og himmel

Jordens duft af
endelighed,
himlens løfte over
dagens forlis

Vågn op, valmuens
kalk vokser 
hjertets vej 

August 2010
                  "),
("Aftryk",558480752672,"Valmuesne

Han knuger vinteren og
klipper saksede
vinger af sneen,
lader frosten sive af
ubeskrevne felter,
drypper varsomt som et
åbent sår, valmuens gåde,
liv høstet for at bære
liv  mod lyset  der
tænder faldne 
stjerner på den 
bundne jord
Han skriver sit 
budskab i den 
hvide stilhed som et
blodblandet løfte


Januar 2010
                  "),
("Aftryk",8590729216,"Vinterhøst

Hvidheden klippet i 
stejle vinkler
Nøgne skrig over
spor i sneen tolker
dagens smerte
Som du sår skal du
høste og den der ikke
høster må plukke livet
hvor livet gror
Røde bær i 
hegnet bærer vidnesbyrd
for himmelfladen 
Så sødt tyder
jorden 
eksistensens gåde


Januar 2010
                  "),
("Aftryk",25904062500,"Vinterliv

Spor i den
sprøde sne
Mand og hund
på tværs af 
hare og som en 
spinkel fløjte,
mus i  skygger
synlig for den der
ved
Havet til venstre
Engen til høje og
himlens altfavnende 
uendelighed
Telefonen i
lommen
Hjertets rytme, 
en underliggende
uforudsigelighed der
griber livet og
jonglerer bolde over
den frosne jord
Jeg ringer, hvis jeg
dør, sagde hun til
datteren og slog en
latter op inden hun
gik med sine
hunde og den
sygdom der ikke 
ville liv


Januar 2010


                  "),
("Aftryk",68786626560,"Åbning

Det er vel i
grunden sådan at 
det man ønsker sig af 
et andet menneske
først kan modtages når 
man selv har givet det
uden tanke på andet
end den andens 
behov


April 2010"),
("Billedbogen",8711667976,"Damen

Da han blev lige så
gammel som damen var
dengang, fandt han ud af  at 
hun var hans farmor og
han vidste jo godt nok det
var hende der 
boede derude ved
bugten i 
huset der lå bagved 
mørket af
træerne de kaldte
hendes  skov
Det var hende der 
legede med dem
ved foreningsjuletræet og de
voksne fortalte historier om
hende som ikke var 
beregnet for
små potters ører men kun for 
de stores og han var
bange for hende for hun
holdt øje med ham lige meget
hvor han var
Pludselig sprang hun af 
cyklen ved skolen og
kaldte ham frem 
foran flokken fra 
hans krog i hjørnet ved 
trappen og han måtte gå
hen til hende og 
høre hende tale gennem det 
rustne net og med ét var hun i 
folkemængden på
banegården i den 
store by på skolens udflugt og
trak ham ud af flokken og
købte varme pølser ved 
pølsemanden i
banegårdshallen og hun 
så på ham mens han 
skyndte sig at spise om 
kap med angsten for at 
blive væk og
hun spurgte om det samme som
hjemme ved skolen og 
altid og han svarede
med øjne der kiggede væk fra
hendes at han da havde det
godt nok og at hans far var 
god nok ved ham selvom en
far jo er noget man 
skal holde sig fra så 
det var da noget
mærkeligt noget at 
sige og han syntes det 
var uhyggeligt at stå 
under hendes øjne der 
ligesom holdt ham fast indtil 
hun strøg ham over 
håret og
slap ham fri
Da han blev lige så
gammel som hun var dengang og
selv hendes navn var 
glemt af  navnene på
kirkegården fandt han
hendes smerte og
kunne se på den uden at
kigge væk 



Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften

Alle fra samlingen Billedbogen




December 2005"),
("Billedbogen",8594153472,"En sommerdag

End ikke vejen til stranden aner 
uråd skønt  poplernes
skygger fanger deres
latter og lader den ligge som 
anelser over det 
skarpe lys

Og langs de høstede marker går de mod  havet med de børn Gud ville give

Gravene husker vel 
stadig kurve fulde af 
barneglæde
De lysende hoveder 
en lærkes tid

De går til havet med
deres børn og fisker
rigdom af de lune pytter

og på vejen hjem
tager den ene retning af en  
forryktheds ørken  og finder aldrig frem

Den anden må tage den 
tomme kulde men finder
nu og da splinter af sol på
bunden af sin
sommerdag





Januar 2005
   


Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften
Skygger

Alle fra samlingen Billedbogen
                          "),
("Billedbogen",214803104009,"Farvefilm

Bedstemor havde fem 
piger og fire 
drenge foruden 
mig der var den 
tiende i flokken 
Kun en var 
yngre  så de ældste var 
jo voksne og tjente 
rundt omkring på 
de store gårde der 
lå som kongeriger 
i det åbne land

Den ældste var 
min mor men
jeg forstod det ikke 
selvom hun kom hjem så 
tit hun kunne og 
forsøgte at være 
det hun var
Hun var så ung og fattig og
bedstemor så stor at 
hun vist havde magt i 
himlen og på hele Guds 
ufattelige jord

Jeg lagde ikke mærke til
at jul og fødselsdag var
mærkelige dage hvor de 
andre børn fik mange 
pakker mens jeg 
vist aldrig sad med 
flere end en men 
jeg hørte voksne tale 
om det og syntes det 
var synd for 
den stakkels lille dreng der 
siden blev forvandlet
til mig selv

Og det hele var i farver og
det var trygt og rart og godt

Vi sad om køkkenbordet og 
spiste pandekager som 
Bedste bagte på et 
sort komfur der 
stod i hjørnet og var  
varmt af ild der 
brændte i 
dets hjerte
Der var en ovn der 
kunne bage franskbrød med 
en varm og voldsom 
duft der 
bredte sig til 
hele huset og til 
gården udenfor hvor 
skraldebøtten stod og 
stank ved siden af det 
lille hus hvor 
vi gik hen når 
vi skulle lave småt 
og stort
Hver gang en pandekage var 
så brun og rund at den 
fik vores maver til at
krympe blev den fanget og 
ført over  på det 
største grydelåg der 
lå omvendt på det 
grønne bord med de 
sultne børn omkring

Og over bordet hang en 
fluefanger der var gul og
våd af lim og rigtig 
dygtig til at 
fange fluer der var 
dumme nok til ikke at 
forstå at man
skal holde sig på 
afstand af den slags
lange gule ting

Og bedstefar der var 
lidt træt -  for han stod
tidligt op - faldt om og 
sov med sære lyde på 
den gamle divan under 
vinduer så 
blå at de 
talte svensk med de
gule vægge der 
var kalket hvide fra 
den brune streg på
midten og hele
vejen op 

Når det blev nat faldt vi i 
søvn i et par
hvide senge men 
når vi sov i 
begge ender og 
lå tæt var det nok

En dag kom et 
flyttelæs og gjorde
holdt udenfor 
På bukken sad en 
mand de sagde var 
min far
Og Bedste græd og 
spurgte om jeg ville op og 
køre med til en 
anden by hvor jeg 
kunne bo og 
være deres eneste  
barn

Det ville jeg for 
læsset var så stort og
farvefilmen gik i stykker og 
blev sort og hvid fra 
dette øjeblik hvor 
jeg hørte hendes ord og
vognen kørte væk fra 
huset med de mange 
børn og bedstemor der 
siden sagde at jeg 
ikke så tilbage men 
blev ved at 
kigge  frem over 
ryggen på de 
store heste der 
trak det høje læs og 
gjorde deres job






Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften
Skygger

Alle fra samlingen Billedbogen
                               "),
("Billedbogen",43490820360,"Guld

Vi løb hen over de 
høstede marker der
lå og lignede guld for at 
fange regnbuen der 
stod med  det ene ben i 
de stynede popler bag  
Langes mark og når
vi nåede derhen ville 
der stå en potte med 
ægte guld der
ikke bare snød som 
markerne der
oven i købet sendte 
kolde skvæt op under 
vores korte bukser fra 
deres fyldte stubbe og 
skar vores ankler med 
skarpe snit
Regnbuen blev ved med at
løbe sin vej men 
i stedet så vi 
fugle der lignede  
flyvemaskiner med 
spilledåser om bord 
Mens vi fulgte morfar langs 
de alenlange  rækker af 
nyfødte roer fortalte han at 
de hed lærker og havde 
reder som vi siden sang om i 
skolen medens fru 
Nygaard holdt os fast med
det onde blik
Da vi kom hjem løb vi 
forbi Mines have hvor man 
skulle gå på tæerne og 
holde vejret mens man så 
de paradisblomster der 
voksede der og 
hed Georginer og 
Dorthealiljer og 
meget mere
Gamle Amanda var død og 
hendes kiste blev kørt til 
kirken på en sortmalet 
vogn der så uhyggelig ud 
med sære figurer og 
små gardiner og to  sorte 
heste foran mens hele 
byen gik bagefter i
deres søndagstøj og 
trådte på de 
blomster nogen havde
smidt hele vejen derhen
Vi mærkede ikke der 
sad en engel og 
skrev det hele op på
indersiden af vores 
hjerter med en 
særligt skarp pen
Engang imellem springer 
sårene op og 
bløder lidt med en 
sursød smerte der 
heldigvis hurtigt går 
over igen



Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften
Skygger

Alle fra samlingen Billedbogen
             
                  "),
("Billedbogen",25774080264,"Hønsegård

I den smalleste
stribe sol under
hækken sidder han og
ser på naboens høns i
den nøgne hønsegård
De skraber med hornede
fødder og finder det som
høns kan lide at 
fylde sig med og
han ser og han ser intet for
det er en af de dage der
har åbnet et rum han ikke tør
gå ind i fordi det er
anderledes end det
plejer at være og
de snakker om ham derinde og
siger frygtelige ting som han
ikke må høre for åbner han
døren holder snakken op
på den måde der 
betyder at der er noget
galt og han ved det er 
ham der er så
forkert at de har sære
ansigter som om de skal til at
græde eller skælde ud
Han forstår at han ikke
er rigtig men hvordan skal han
ellers være han prøver jo hele
tiden at være så god at Bedste
stryger ham over
håret og siger at 
hvis nu alle var lige så 
gode som dig så var
verden et godt sted at 
være men det er den jo 
ikke for han er alligevel ikke så
god at de er 
glade og de andre børn synes 
også han er 
sær og Bedste er jo 
alligevel ikke hans 
mor som hun er
mor for de andre og 
de spørger på gaden
hvem der er hans far og 
så ved han ikke
hvad han skal sige 
og tænker på den gang han
hørte en af de andres 
far sige min dreng imens han 
strøg ham over 
håret og sagde har I det 
sjovt, min dreng så der 
ikke var noget at være 
bange for der og en af 
hønsene har fundet en 
orm som hun prøver at 
trække op af jorden men 
straks kommer kokken og 
jager hende væk med et 
hug så hun taber en 
fjer og måske kan han 
nå den hvis han lægger sig på 
maven og 
fisker med en pind



November 2005


Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften
Skygger

Alle fra samlingen Billedbogen
                               "),
("Billedbogen",8594473224,"Juleaften


Juleaften fik vi 
risengrød
af de store gule 
tallerkener med de 
små røde roser som 
ellers kun kom frem 
når mormor havde kogt 
hønsekødsuppe med 
2 slags boller
Ikke bare en lille klat men
en hel skudefuld
der skulle lægge en 
god bund inden 
flæskestegen der 
duftede langt hen ad 
gaden
Og der var en mandel i 
men først skulle man 
spise sig gennem hele 
portionen og måske 
en til 
inden nogen fandt den og 
blev ejer af den lille grå 
marcipangris
Vi var ikke i kirke først 
for det gjorde vi ikke
men vi cyklede 
forbi på vej til 
mormor og så lysene 
skinne ud i mørket
Juletræet var skævt og 
nogen havde bandet længe 
inden det stod på 
sin fod og kunne 
pyntes med lidt 
lys og musetrapper 
en håndfuld fehår og 
et par glaskugler der 
havde overlevet 
tidens gang
Forlegne sang vi 
vers af salmer med 
øjnene begravet i 
en salmebog
Og så var der 
gaver
Et par strikkede
strømper
og hvert andet år en
julemand fra
Sommers Magasin
Inden vi kørte hjem
havde jeg
revet skægget af
for at se
hvordan han så ud
uden



 
Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften
Skygger

Alle fra samlingen Billedbogen
             "),
("Billedbogen",9932111872,"Lysjagt

Mellem husene
går skyggerne på
jagt efter 
uopmærksomt
solskin 
Stjæler glans fra
ubevogtede farver i
lysfyldte flader
Rækker over 
gaderne og
kravler op ad
fremmede facader
for at slikke sol af
tåbelige vinduer der
tror at mørket
bliver udenfor hvis de
lader som ingenting
Som forløbere for
natten ruller de sig 
helt ud som villige veje for 
det store mørke  der tøver et 
øjeblik endnu inden det 
skrider ind og slukker den 
sidste glød 


                  "),
("Billedbogen",8725200896,"Mirakler

Dagligt falder
himlen ned på
jorden uden at  
det bemærkes
Engle går lyslevende rundt og
hvisker ord i ører der ikke
kan være foruden og
på udvalgte steder slås op i 
bøger med glemte linier der 
springer ud og kaster sig 
i synet på søgende der 
vantro hæver blikket og 
forstår
To mennesker mødes efter
mange år og ser ikke
det smil der falmer væk på 
undersiden af en  
sky der har travlt med
at dryppe lyse ideer over
træge hoveder
Med travle sanser justeres
skjulte kredsløb der
retter på de værste 
fejl vi har lavet siden 
sidste gang vi 
mødte himlen og 
fik hjælp fra
oven til at 
starte på en frisk



                  "),
("Billedbogen",8594210824,"Onsdag


Fiskemanden kom
kørende den lange vej fra havnen
hver uge
med en fiskekasse bagpå 
og en anden foran
Fiskene lå levende og gispede efter
vejret mellem 
smeltende isstykker
der dryppede vådt under cyklen
Konerne kom frem af husene og lo med ham
mens han med øvede hænder flåede
fladfisk eller talte sild op
Konerne havde forklæder på og fiskemanden 
tørrede hænderne i 
en gammel sæk
Unger i alle aldre suttede 
isstykker, de så deres snit til 
at stjæle fra kasserne og
sommeren var altid hed
og himlen altid blå i den særlige farve
barndomshimle har
Om aftenen lugtede der 
af fisk fra alle køkkenerne helt
ud mellem æbletræerne
der stod forladte af 
jungledyr og sørøvere mens de 
dryssede blomsterblade i 
den lune aften
Senere kom måndende ud 
på trappestenene hvor de sad og 
røg deres piber
mens man hørte opvaskelyde
indenfor
Længere hen på aftenen når 
verden var blevet
alfestille
skinnede månen gennem 
vinduet der tegnede 
et flot kors mod 
lyset og én sagde engang at
sådan måtte det være når 
man var død og lå 
og kiggede op af 
graven.


 

   
Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften
Skygger

Alle fra samlingen Billedbogen
             
              "),
("Billedbogen",8589934592,"Parker

Vi går gennem
parker hvor de
vilde duer kurrer
fra gamle jerngitre der
varmer nøgne guder og
gudinder 
Vi springer buk med
efteråret og sender
raslende blade tilbage til
himlen de kommer fra
med små inskriptioner til 
ham der tager imod
Under en blå
paraply er solen ligeglad med
regnen der venter på
lov fra en anden dag hvor 
vi har lyst til at 
lade os gennembløde til
skindet af en fordrukken 
sky der brovtende farer hen 
over himlen og spiller
tyfon over haver der
dukker nakken og slubrer
dråber for at kunne
blomstre på ny



                  "),
("Billedbogen",8725200912,"Sangen til solen

Fortovene holder sig
tæt til husene og 
kigger ligeud
Gaderne løber af sted i
alle retninger og leder efter 
veje der fører til
Paris og Nordkap eller 
andre steder som
ingen har hørt om 
endnu
Vinduer ingen har
tal på kravler op mod
himlen og lukker 
lys ind og
holder på varmen mens 
de forsøger at se hvad 
der er at se uden at 
miste facadens
værdighed
Helt nede ved gaden 
holder de pyntede til 
der  skamløst viser sig 
frem og har priser på 
alt der kan fås
for en pris

I sprækken mellem
to løse sten vokser en
gylden blomst der er 
brudt ud i sang til 
solen der gemmer sig bag
tagene og lader som om 
den ikke er 
hjemme selvom det
er den der holder det
hele i gang og 
sørger for at
alt går rimeligt til





                  "),
("Billedbogen",283468890128,"Æblemåne


Det er aften og 
mørket kryber frem under 
æbletræet der 
ryster sine 
vinger en sidste 
gang og drysser hvide
blomsterblade over 
skyggen
Månen kommer
frem af sit skjul
og kravler op
på plads
hvor den smiler
let besværet
Det er tydeligt
at den er fuld
Englene har travlt bag 
himlens mørke
bue
De fjerner skodderne
så man kan se 
lysene i Edens Have 
Og ud af alle kroge vrimler 
sjove dyr der flyver rundt 
og brummer på 
lange lodne vinger


                  "),
("Brønden",0,"1963-64

Jeg vidste ikke, det var digte.
Jeg skrev bare, med min fyldepen, 
i den tykke linierede bog, der efterhånden blev efterfulgt af flere.
Det var først da forfatteren Henning Ipsen - 
som jeg kendte overfladisk -
så dem og forbløffet sagde: \"Jamen du
skriver jo digte\", at det gik op for mig, at det var det jo nok.
Han gav mig herefter et så kraftigt skulderklap, at jeg siden har skrevet med 
en vis frimodighed,
løsrevet fra ethvert forbillede og enhver
stilstrømning."),
("Brønden",50372608,"At være 

At være et menneske 
Alene 
At være et menneske alene 
At være indespærret 
At være indespærret 
i sit eget univers 
Kontakter over skellene 
Universer tørner sammen 
Åbner sig 
Lukker sig 
Tiltrækkes 
Frastødes
                  "),
("Brønden",85916155904,"Bøjlemand 


Han var en af dem 
der burde hænges 
på bøjle 
og gemmes væk med en hvid 
mølkugle i lommen 
En varm forårsdag 
kunne han blive
taget frem og 
promeneret 
på gaden 
          "),
("Brønden",5275912,"Barndomsminder


Hanebjælkespær 
Mørkehyggerædsel 
Stråhalmknitterleg 
Døsigvarmeem 

Kriblekrillespænd 
Næsenysefrem 
Latterfangehyl 
Barnemindefryd 

                  "),
("Brønden",0,"Besættelse 

Et sugende gab 
åbner sig 
i kraniets 
benhvide flade 
Evnen til selvstændig 
tænkning 
lammes gradvist 
og forsvarsløs 
møder du 
de indtrængende 
voldsmænd 
der uden at 
forhaste sig 
tager ophold 
i dig 
og 
forstener 
dit ydre 
Dine fortvivlede skrig 
din ynkelige bøn 
om hjælp 
får ingen chance 
for at slippe 
ud 
af det rædslens 
torturkammer 
der vokser op 
i dig 
Du forsøger 
hjælpeløst 
at finde en 
vej ud 
uden at lytte 
til den djævelske 
hvisken 
der uophørligt 
fortæller om 
den ene 
udvej - 
den ene 
mulighed for 
flugt 
du aner 
uundgåeligheden af 
De hårde 
ansigter 
smiler roligt 
mens blanke 
instrumenter 
lægges frem 
i faglig orden 















          "),
("Brønden",17213431808,"Billen 

Tallerkenkant 
dyr med striber 
bille 
joller sindssvagt 
rundt og rundt 
Standser 
vejrer 
pudser lange antenner
Seks sorte ben 
går atter 
løbende igang 
og lægger nytteløst
spor over spor. 
Fremad 
Fremad 
Retning 
mod 
sig selv.

                  "),
("Brønden",131104,"Blå fisk


Blå fisk
med gabende mund
står dødøjet
på lur
i mørket.
Finnerne dirrer
umærkeligt parat
til bevægelse
angreb
død.
Livet kræver
et bytte
nu og da
og får det.
Den blå krop
bliver næsten sort
når døden høster.


                  "),
("Brønden",0,"Bogstaver 

Sorte bogstaver 
kravler vrimlende 
nedad avisens forside 
Aftegner i et nu 
et mønster 
der et øjeblik senere 
er opløst 
og atter samlet 
i en ny 
kalejdoskopisk broget 
enhed 
Som en sort flod 
glider de ned ad papiret 
for at falde til ro 
i glemsel og fortid


          "),
("Brønden",8610906112,"Brønden


Kanten
af en brønd
Langt borte
det mørke vand
der dunkelt
gengiver himlens farver
Ekko
mod slimede sten
Mit ansigt
dukker frem
Sløret i trækkene
Stirrende
Himlen kalder
og vandet lokker
dybt dernede:
Kommer du
Kommer nu
Lade falde
Sove
Mørke, mørke
Blive tung
Nu eksisterer kun
en dybsort flade
der med et
åbner sig
og vækker indestængte kræfter
Mørke
Drivende skyer
Øjne
Et par stive
stirrende øjne
Mine
Mine
Jeg
Jeg fryser
Rejser mig
Ser"),
("Brønden",0,"Dans dukke


Dans dukke
Plasticben
Neonlys
Slimet musik.
Dans dukke
Liderlig
rødskjoldet
tom
erotik.

                  "),
("Brønden",0,"Farvefantasi


Hestene danser i sneen 
med blå guldsko 
på de sparkende hove. 
Frosttågemankerne 
hvirvler omkring hovederne 
som sølvsnore 
mens tungen 
rød som blod 
lyser frem 
mellem krængede læber.
          "),
("Brønden",10758390033,"Fiskerbåden


Jeg vidste det ikke 
dengang 
men noget skete med mig 
da sennepsmarkens 
dufte 
blev ét med 
nattens skære 
kølighed. 
Vibens hæse skrig 
trængte dybt ind i mig 
og blandedes 
med frøernes koncert 
ved fjorden. 
Og da morgenen kom 
listende 
hørte jeg 
en fiskerbåds 
dybe hamren 
langt inde 
i mig. 

                  "),
("Brønden",4194304,"Fodspor


Cirkulære baners 
uafvendelighed 
gulner 
givtig 
grøde. 

Som fodspor 
i det våde sand 
ved kysten 
slægters stille 
stræb"),
("Brønden",0,"Forord 


Talløse fordybninger 
Tavse vers 
der som 
sten er 
lagt ud i 
bække 

Hvide bruskkegler 
Brudte tegn 
Høje træers 
blåsorte skygger 

Søgende toner 
Flimren af liv 
visende vej 
til brøndene 
i os 


          "),
("Brønden",16809984,"Fortvivlelse 


Lyset forsvinder 
i en sugende malstrøm 
hvis centrum 
hastigt 
rykker nærmere 
Dagslysets lykke 
bortjages 
af nattens rædsel 
Rædslen for mørket 
og ens eget jeg 
Sandhedens stemme 
råber spottende 
håner 
med sårende ord 
Sandheden
hvor er dens rod? 
Sandhed for dig 
er måske 
løgn for mig 
Vinden rejser sig tudende 
Regnen strømmer ned 
og bliver til is 
Sindets landeveje 
brolægges med faldne lig 
i tusindtal
Et øjeblik 
er der stille 
men stormen kommer igen 
Den vender altid tilbage 
og mørket vinder frem 
Skal vi da evigt flygte 
Evigt være på vej 
mens vore fødder tramper 
over skeletagtige døde? 
Skal lykke altid 
forjages 
af skyggerne 
der halter 
gennem stormen 
At være til 
men ikke leve 
Være åndepust af liv 
Så sig det bare 
Råb det højt 
så alle mørkets skygger 
rejser de indfaldne hoveder 
Sig det mens stormen hvisler 
så ordene bliver et 
med blæsten 
Sig 
Det er forgæves 
for mørket er dig selv 
og regnen har du i dig 
Du var din egen lykkes smed 

          "),
("Brønden",68753063936,"Fredag aften


Vi mødes 
smilende 
rundt om 
bordet der 
altid er 
imellem os 
med sin 
sirlige 
opdækning 
af blankt porcelæn. 
Vi fortæller 
spisende 
om vore umistelige 
verdener 
ukuelige 
som telegrafister 
der forsøger 
at trænge 
gennem universet 
uden at få svar. 


          "),
("Brønden",8591015936,"Glæde


Jublende danser jeg 
på svævende broer 
med solen i maven. 
Perlende bobler 
fra mit inderste indre 
drives ud 
i kaskader 
af lysende stjerner. 
Hurtigere 
Hurtigere 
til den blå luft 
tager mig 
og hjertet 
brister med 
glaslyd.
                  "),
("Brønden",18270421024,"Guernica


Kender du Guernica
det gråsorte billede
han har lavet
den gamle mester.
De løsrevne dele
af levende væsener
flyder rundt mellem
underligt fremmede
følelser i mig.
Lampen foroven
lyser dybt ind
i mit indre
og viser mig ting
jeg aldrig har anet.
Menneskenes forvredne
ansigter
afspejler dele
af min sjæl
jeg først nu
kan erkende.
Længe
har jeg undrende
stirret
i spejlet.


                  "),
("Brønden",68786618368,"Hænder 

Hårde ord vælter strømmende 
ud over læbernes lukke 
Hjertet skælver af angst 
Øjnenes stadige flakken 
ser fornærmende koldt 
Pinende usikkerhed 
Hænderne skælver 
en forbandet rysten 
der ikke vil holde op 
skønt jeg vil og vil 
Hvorfor ser du ikke min angst 
men bliver stiv og kold 
Se på mine hænder 
De græder under dit blik

                  "),
("Brønden",8594391040,"Høstregn


Hidsende tungt 
lader regnen 
sig falde 
mod byen 
der grådigt slubrende 
tager imod 
Skyernes masser 
saftspændte 
tunge 
hviler 
vulgært
på 
de strittende tårnes 
opstrakte 
fingre 
Væden flyder 
drivende 
Sprøjtende tove 
af lunkent vand 
Mørkere 
Vildere 
Trommende 
Plaskende 
Til mulmet flænges 
råt 
af et glimt 
Ildhav 
Ildhav 
Væltende vogne 
der dundrende 
raslende 
falder til ro 
mens tagene drypper 
og pytterne 
sprøjter 
når bilerne 
kører forbi



                  "),
("Brønden",268435472,"I mørket


Under bladene
af en stor skræppe
fandt jeg en tusindfryd.
I mørket
var strålerne fuldendte
og tusind små hår
på bladenes flader
fik mulmet
til at gløde.

                  "),
("Brønden",0,"Insekt


Pergamentagtig hud
udspændt over
en glødende pære.
Lysskær
Matglinsende edderkoppeben
Leddelte
Famlende
over hvide ansigter.

                  "),
("Brønden",17196777488,"Kamille


Udgravning. 
Jordvolde 
nøgne og golde 
som mennesker 
der af skæbnen 
er åbnet og blottet. 
Hvide kamiller 
som afværgende floskler 
forsøger 
at illudere 
frugtbar grøde. 

                  "),
("Brønden",0,"Kedsomhed


Tasken
blev træt
af skabet
og rejste sig
pludselig
og løb væk.
Manden og konen
løb efter den
råbende
men tasken
løb væk.

          "),
("Brønden",8730345473,"Krager


Solskin på nøgne marker 
Krager på værdige ben 
inspicerer 
Frostkold morgen 
Flammende hyben i hegnet 
Sorte grene i nøgen kontrast 
Blåkalket himmel

Livet får aldrig ende 

                  "),
("Brønden",135266304,"Kredsløb


Født
på en vandrende klode
hvor kredsløb
er livets mystik.
Brune blade var grønne,
var knopper,
der glinsende hang i solen
bristefærdige
som unge piger,
før de springer ud.
Brune blade rives af
bliver muld.
Unge piger
ældes til muld
der genskaber liv og kraft.
En ring er sluttet
når knopper brister
og sjæle fødes til børn.
En gylden ring
som aldrig brydes
på sin
regelrette vej
gennem universet.


          "),
("Brønden",4299161600,"Landsbykirkegård


Månelysets 
elfenbenshvide skær. 
Buksbomhækkenes 
grådkvalte dufte. 

I skarptskårne skygger 
de vuggende sjæles 
mægtige 
vokstulipaner. 

Regnormes slimede spor 
tegner lyse streger 
på de 
grusbelagte gange.


                  "),
("Brønden",0,"Livsløgne 

Det passer jo ikke 
hvad slægterne hvisker 
Granernes grene strækker sig højere 
end menneskesatte idealer 
Det er uværdigt at leve 
på klatrende løgne 

Det er en flugt 
fra de afstukne stier 
at gå rundt i små kabinetter 
hvor ægthed er anbragt 
side om side med 
primitive folkeslags amuletter 

Det er for sent at angre 
når man har valgt 
Indestængt af løgnes spind 
sidder vi som fluelig 
Forklaringer der ligner bøn 
fylder luften med summen


          "),
("Brønden",8774483968,"Længsel 

Det ensomme menneskes 
drivværk 
er længslen 
der langt inde 
bag kødet 
nervøst fingererer 
med håbet 
om andre tider 
Selv på dage 
hvor skuffelsen 
tilsyneladende 
har slukket 
livsviljen 
trækker længslens 
fjeder 
livet pulserende 
gennem årerne 

                  "),
("Brønden",0,"Opskrift


Efter den obligatoriske 
odysse gennem 
udviklingshistorien
fødes du effektivt 
efter de herskende 
psykologers nyeste 
metode. 

Du vokser op på 
den rigtige ernæring 
tilsat netop de 
stoffer 
vismændene 
i de sterile laboratorier 
har forordnet. 

Din sjæls udvikling 
er komputerstyret 
og ledes nænsomt af 
kompetente pædagoger 
der også foretager 
sorteringen 
til de forskellige 
rangklasser. 

Specialopgave og løntrin 
programmeres ind og du 
lever livet efter 
nøje fastsatte 
regler. 

Erfarne dødshjælpere
sender dig 
rutineret videre 
når kroppen må opgives. 

Religionspsykologerne 
skriver de sidste 
bemærkninger 
i din journal. 

Modstræbende 
lades du 
alene. 



          "),
("Brønden",0,"Ordet 

Ordet 
var 
og er 
som du 
Alt andet 
kan glemmes 
          "),
("Brønden",68753031168,"Privat


Privat cirkel
Privat radius
Andres private periferier.
Stive voksdukker
Automatik
Formelsamvær
Receptvenskab
Livslede
Løgn.


          "),
("Brønden",16777216,"Program 

Erkende og 
forstå 
Arbejde med 
mig selv 
Forstå dig 
og derfor holde af dig 
Lade dig forstå mig 
Men følge mit eget jegs 
nødvendige veje 
          "),
("Brønden",50364416,"Rædsel 

Rædslen 
er parat 
til at krybe 
ud af øjnene 
på mig 
hvad øjeblik 
det skal være 
Himlen lugter råddent 
og jorden under mig 
stinker 
som var alt 
i opløsning 
som var alt på vej 
til at omdannes 
til ildelugtende 
luftarter 
Mit indre 
er for længe siden 
ædt af orme 
og opløsningen 
af min skal 
er så langt fremskreden 
at også jeg 
stinker 

          "),
("Brønden",138444800,"Skærsommernat


Majgrøn
træbro
til at gå over.
Silkeskørt.
Majgrønne
dugdampelyde.
Djævelsk latter.
Majgrønne lyde.
Farlige fristere.

                  "),
("Brønden",2097152,"Sofastykke


Bølgeslag mod kysten.
Tudende storm
i øret
der smerter.
Måger
der flyver skrigende
omkring
og varsler undergang.
Havet snerrer hadefuldt
og viser hvide tænder
mod landet
der forsagt skyder ryg
og lukker sig
i angst.
Himlen er fuld
af farver
der ikke kan males
med ord
men kendes af alle
der har følt
rædslen
for det uafvendelige.
Havet er i oprør
og landet skælver
som vi vil skælve
når mørket bryder løs
som barnet skælver
når far og mor er borte
og alt
er uforståeligt
og mørkt.
Et barn kan græde
men vi ved det ikke nytter.
Derfor tier vi
og fortsætter
leende ubekymret
til hinanden
som var vi glade.


          "),
("Brønden",8595439872,"Sommerglæde


Sommerskinssolglæde
Yngsterlegfryd 
Øjesmutblidhed 
Fløjelsvarmesind 

                  "),
("Brønden",4194304,"Stationen


Jernbanestation 
Gråsortblæsende 
menneskefyldte 
neonlysperroner. 
Lattergrådråbende 
metalplakatfløjtende 
prustelyde. 
Togstammeskæbner 
Afskedsubønhør 
Rudefarvelglimt 
Hvidvinkehilsener.

                  "),
("Brønden",50331648,"Stilhed


Dagen bliver stille
i mørkets skygge.
Lydene dæmpes
som af et tæppe.
Er man ene
hører man rummet suse.
Men tankerne sover aldrig.
I stilheden vælter de frem
af sære forvirrede baner
der alle mødes i hjernen
og kræver svar.
Så flygter man
måske
fra stilheden
og sig selv.

          "),
("Brønden",17184063488,"Sven Erik 


Pludselig stod han her 
i døren 
min barndoms kammerat 
som jeg næsten havde 
glemt 
Da han rakte mig hånden 
til hilsen så jeg med et 
at han var blevet 
plastic 
Kvalmen rørte rundt i maven 
da jeg så på ham igen 
Det var jo ikke Sven Erik 
Det var ikke engang 
et levende væsen 
men bare en tom skal 
med en glat uangribelig 
overflade 
Han talte længe med en stemme 
der heller ikke var hans 
Da han gik stod jeg længe 
og så efter plasticmanden 
der engang var Sven Erik 


                  "),
("Brønden",33587200,"Tiden


Urets tikken får luften
i værelset til at
gløde.
Mit hjerte smerter
under lyden.
De arbejdende tandhjul
knuser mine knogler
mens de sejgt
æder tid.


          "),
("Brønden",17230233600,"Tom


Han fyldte tredive 
med tungen 
hængende ud af 
halsen 
som 
efter et styrtløb. 
Over mållinjen 
opdagede han 
tilskuerpladsernes 
tomhed 
og blev angst. 
Den uendelige 
cirkelbane. 
Hjertets desperate 
hamren. 

          "),
("Brønden",8192,"Urfødelse 

Siddende på toppen 
af bjerget 
varmer vi 
vore kolde hænder 
ved den flammende ild 
Over vore hoveder 
gror lysende punkter 
plantet af fremmede vinger 
Vi taler ikke 
men spejler 
bålets flammer 
med livløse 
stirrende øjne 

En efter en 
vender vi os 
mod lyden 
der langsomt vokser 
Vore skygger 
står tegnet skarpt 
i hvælvet 
men ingen af os ser 
Lyttende står vi tavse 
og fryser de bortvendt ansigter 
uden tanker eller bevidsthed 
som støtter 
med hængende hoveder 

En efter en 
går vi bort 
ned 
i det ukendte mørke 
Blindt søger foden 
de fremmede stier 
mens lysskæret følger 
på vej 
med lange dansende stråler 
En efter en 
svinder vi bort 
i det kolde 
kaldende øde 
vi ikke kender 


                  "),
("Brønden",50364416,"Var det mig? 

Jeg har brug for 
at føle nu 
men kan 
ikke 
En dyb 
undren 
en søjle af røg 
vælter op 
fra dybets kaos 
som et spørgsmålstegn 
Var det mig 
jeg så 
dernede 
med det hæsligt 
forvredne fjæs 
og de løftede 
hænder 
der forsøgte 
at nå lyset


          "),
("Brønden",21102608,"Var jeg maler


Var jeg maler
valgte jeg
den dybe blå
og smurte den
langt ud
i havet.
Sennepsmarkens
skære gule
skulle lægges dybt
på skrå
forneden.
Signaturen
skrives
i det højre
hjørne
tæt ved kanten.
Jeg har gjort det."),
("Brønden",0,"Ventetid


Den tavse luft sitrer.
Dovent glider tiden afsted
og æder skæbner.
O Gud, jeg dør af lede.
Jeg sidder og venter stille
mens mennesker omkring mig
haster afsted i flimrende dødsangst.


Jeg har levet i tusinde år
og forstår alligevel intet
af dette.
Mest uforståeligt er mit indre
der råber protesterende
skønt jeg ikke kan tale
og sidder stille og ser.

Længe har jeg ventet på
at min skal skulle sprænges
af en tordnende stemme
der med et slag
ville få alt til at standse.
Skal jeg vente 
hvor længe endnu?

          "),
("Brønden",0,"Vold 


Lysstump 
snart udbrændt 
Stivnet stearin 
Organiske former 
Den sorte væge 
med spor af vold 
Mordbrand 
Sugende 
brændende
væsker 
Ubønhørligt 
Det er forbi 
          "),
("Brønden",68787666944,"Ømhed 

Så let som et dun 
blev min hånd 
da jeg sagte 
strøg dig over kinden 
Så blød 
som en killings pels 
så varm 
som et strejf af sommer 
hvælved den sig 
under min hånd 
der på en gang 
blev uendelig stor 
som for 
helt at omsluttte dig 
gemme dig 
værne dig 
Dog 
forsigtigt 
som var du en sommerfugl 
fjerned jeg hånden 
for ikke 
at tørre støvet 
af de sitrende vinger 


                  "),
("Dagen",552171732992,"Ønskemorgen

At plukke dagen med
rolige hænder og bringe den
sikkert i hus

Mørket har det bedst
udenfor

Det lys du har tændt
skal ses af alle der
færdes derude

                  "),
("Dagen",8594259968,"Blå

De små hvaler er
meget for
blå 

og himlen

svæver løst mellem
vilkår 

over bundens 
egensindighed

                  "),
("Dagen",1048576,"Den første sne

Den første sne
har drysset jorden
hvid og ligger og 
venter på 
fodspor der går i
lange linier 
forbi        af sted        hen over   og 
måske 
            tilbage     inden 
de danner mønstre og 
krydser andre spor
eller mødes og 
følges ad i
samme ærinde eller 
bare nyder at 
gå ved siden af 
hinanden mens
glæden over ingenting sejler 
ned og farver forslidte 
tanker hvide og
former nyfødt  liv af
sommerens kasserede
rester       

                


                  "),
("Dagen",8598323200,"Facader


Huse
svøbt om
indhold
Rum om
territorier hvor 
vægge og lofter 
hvilende på 
gulve danner
omfang for 
liv
Stemmer eller ingen
Lyde af liv eller 
stilhed
Dryppende dråber og
stemninger der 
danser med 
følelsesstorme
bag den onde 
tavshed
Dufte og falmende
fotografier
af facader
Udvendige portrætter
af liv der
kigger ud 



                  "),
("Dagen",17230233600,"Fernis

Bag de bemalede
skærmbrætter
synger fuglene ikke
men lytter til
livet der preller af
mod sarkasmens
glatte flader
          "),
("Dagen",134217728,"Forløb


Stregers begyndelse og
afslutning
Flænsende lysspors
forløb mellem
punkter modsiger 
mørket i 
lange koder over 
graves ekko af
barnegråd
Dynastiers talrækker
løber gennem 
epoker udstrakt af
skabningers
fødsel og død
Flammende
viljer på vej skriver 
spor over nattens
jagende vinde


          "),
("Dagen",134217728,"Form

Der er lyset og 
der er mørket der 
kravler frem i læ af
skygger
Der er ordene og
der er tavsheden der
vogter sit
indhold tæt
Det sete der 
dækker det usete
Støjen der ruger på
stilhedens tikkende
æg

Der er forløsningen og
den opstemmede
energi der
modner frugt
Sorgen og
gråden der gemmer
smerte bag det 
synliges manifest

Der er livets elskov
med døden
Handlings og stilstands 
udelte magt
Glæden og lysten der
springer ud i
forår midt i  
ørkeners brændende
foragt


          "),
("Dagen",1207959552,"Fremskridt

pågående

ligegyldige informationers 
enerverende inklination til
dans over sandheders
skjulte tilholdssteder

analyser

forklædte sensationers
krysantemumbrøl mod den
mørke himmels 
tavshed

bekræfter

på tværs af
grundlove vokser
det evige fremskridt

påstande
          "),
("Dagen",8724316160,"Grå

Byen er indædt
grå
gaderne 
husene 
himlen

Byen er
befolket af
grå  der går forbi 
hinanden eller standser op og
taler med monotone
stemmer om grå
hændelser der løber op og 
ned ad de grå facader
Butikker fladtrykte af
deprimerede varer der er
set før og kun til stede for
at få en handel i stand

På et torv står et træ og 
brøler til himlen med
gule flammer


                  "),
("Dagen",1073741824,"Høre, se, tale

Øen ligger badet
i umuligt forår
De gyldne skrænter
spænder himlen og
havet op i et 
blå båret af 
lærkesangen der af 
ingen verdens
grund pibler frem af 
kilden i mit 
hjerte
Fjerne byers brændende
tage slører 
horisonten der ikke 
vil høre
ikke vil se
ikke forstå sin
stumme tale


          "),
("Dagen",136314880,"Havfruedrømme

Polerede vandflader
spejler den 
blikbeslåede
himmel og lægger
låg over
havfruedrømmes
skjulte liv
Svævende
gallerier
rejser omfang  over 
bundens greb om
den nøgne
sandheds jagt 
på vækst 
Magtfulde rørelser
gennembryder
stilstandes lukkede
stabilitet
Sange over sunkne
byer gløder
mørket
          "),
("Dagen",69826775040,"Jerusalem

Bundne sneskreds 
forløsning gløder i
vidende øjne der
vagtsomt vogter
øjeblikkets ro før
kroppens bratte
bevægelse udløser
viljers opstemmede
energi

Stilheden over
bjergets side
tøver over
ofret liv


          "),
("Dagen",33280,"Klip

Lynskarpe klip i
tid lader
sammenrullede
aggressioner
folde sig
huggende ud
gennem årenes
bogførte orden
Splittede kolonners
forvildede jagt på
decimaler dækkes af
facaders understøttede
urokkelighed 
Nye mønstre  af
slidte brikker
søger sammenhæng


                  "),
("Dagen",134217728,"Kontrol

Strømmenes flydende
svæv mod
forløsning følger
de givne baner i 
logisk balancerende 
lydighed

Opstemmet kraft
sprænger vej
gennem allerede
forberedte muligheder
uden hensyn til
konventioners drøm om
kontrol

Rørlagte lidenskaber
lækker gærende
råddenskab i
slimede pytter
          "),
("Dagen",0,"Minutmarchen

Minutterne kommer forfra og 
står på række for at 
komme til
De lange og de  korte
De velkendte der 
aldrig har esser i 
ærmet 
De grå mellem de 
kulørte og dem med 
fuld musik
Der er dem med 
bedemandsfjæs der 
prøver at trække tiden
ud og ind imellem
gemmer sig en 
lille trold der har 
forklædt sig og  
håber på  at blive 
pakket ud
Der har været nogen
der har standset
rækken så længe at
tiden troede den
var gået i stå

Engang imellem er
der nogen der kommer
tilbage og står og prikker
en på  skulderen for at 
få opmærksomhed
Dem skal man bare
lade stå så de kan lære at
når man er gået så er 
man gået for der er 
hele tiden nye der
har gjort sig umage med
at være parate når
det dem der har tid



          "),
("Dagen",2416967680,"Nu

Uantastet over
horisonten
fastholder solen
sin position og
tænder beslutsomt
viljen til liv i
regioner der for længst
har kapituleret 

Tiden er inde

Det nytter at
kæmpe



31-12-2003

                  "),
("Dagen",8388612,"Revner


Fine revner
løber op ad 
husenes facader og 
vidner om et 
forfald hoveddørene ikke
vil vide af
Gennem forgrenede netværk og 
siksakmunde varsler 
kommende kløfter 
adskillelse og destruktion af 
sammenhænge der troede sig
sikre
Undergrundens uforløste
spændinger jamrer sig 
vej frem i 
lyset hvor de 
uantastede 
bygger katedraler af 
smuldrende planer over
knuste drømmes
monumenter

          "),
("Dagen",68719510016,"Roser

Roserne blomstrer
udenfor mit fejede og
prydede hus
Duften fortrænges af
sorg fra andre huse der
er og var engang
Uforløste følelser hopper
over hækken og
tørrer jorden ud mellem
mine ømmeste punkter
der må vandes igen og
igen
Skrigende erindringer
slår ned i de højeste buske
med baskende vinger der ikke
et øjeblik er i
ro og ustandselig ringer det
på døren mod gaden hvor
ukendte påstår de er mine
nærmeste der er kommet for
at indløse gammel gæld jeg
har stiftet ved min
eksistens

Jeg fejer
pryder
og vander mine
roser

                  "),
("Dagen",33554432,"Skakter

Himlen over
husene
belyser de
nøgne kanaler for
hjemløse lyde der
må have et sted at
gå hen
Kun et par
øjne uden
mål
ser til
          "),
("Dagen",1073741824,"Skammen

Skyggen vokser sig 
over vejen og 
stjæler lys fra 
landskabet der
forsøger at samle
kraft
Det bliver mørkt nu og
vejen løber over
bakken og bliver
ikke fulgt
Stilheden tøver men
finder ingen fred


          "),
("Dagen",0,"Skrift

Stranden er fladen
Himlen er kuplen
Kuplen over fladen der
rækker til havet 
Havet  der dækker sin dybde med
lyset splintret i 
planer

Vandrende skyggers
glidende baner
kærtegner fladens
modtagelighed

Kupler over
flader
Fladers modtagelige
hud beskrevet med
vandrende skyggers
flygtige skrift






          "),
("Dagen",512,"Smittefri

Under ordenes
velholdte flade
løber den 
urensede strøm gennem
båndlagte rør der
holder sammen og tæt

Filtre og overløbsbassiner
regulerer kriser og lader
smittefri udtryk løbe sammen
i havet der uden armering
ånder lysets leg til live




                  "),
("Dagen",68719476736,"Spillet


Sønderrevne
fosterhinders tid
lader lyset hejse
illusioner der
som spørgsmål
ligger åbne mellem
afstemte flader
Indlærte svar
kastes ud over
rampen til rette tid
af programmerede
dele af enhed
der lader spillet gå
til ende
Øjne ser gennem
masker



          "),
("Dagen",16777216,"Svar

Opløste forhåbninger
forenes i søen der
kaster et ansigt tilbage som
eneste svar
          "),
("Dagen",1073742336,"Tåge


Tågen opstod
Er

Lyset 
besat af grå
stilhed
En tom tvivls
vakuum hvori
navnløse skygger
fra dybet
drager forbi
En hær af 
stumme automater 
í march uden 
destination
Dagens vågne 
drømme i
retningsløs
fortabelse
Den våde kulde
mod et ansigts
lammede træk


                  "),
("Dagen",135266304,"Tanker


Bakker og dale
drysset med
tankegods
Himlen arret og
ridset af
opsendte drømme
Universer på
svajende stængler
sender skyer af
impulser op over
jordens villige
grobund
          "),
("Dagen",512,"Vandflader

Lyset over de
gyldne vandflader
afslører ikke
bundens tilstand men
spejler villigt
himlens drivende
skyer

Blide vinde antaster
ikke forrådnende
dele af skamløst liv der
aldrig blomstrede
åbent

Fuglesang over blinde
spejlflader overdøver
de navnløses gråd"),
("Landskab",50373152,"De

De dør omkring mig
Falder om og bliver til
Noget
Ligger hvide i senge og
svinder ind 
Er ikke mere
Deres navne står 
på sten på kirkegårde
Eller blot en dag i 
Avisen for at forsvinde
mellem ukendte
De går bort
De får fred
De tager hjem
De er ikke mere
mens jeg stadig
går her 
ligesom
uberettiget for
Hvor går jeg hen og
burde jeg ikke være
sammen med det store
flertal i 
tavs solidaritet
Det er noget at tænke over
En livsopgave der
kræver alle mine
skridt


22.10.2013

                  "),
("Landskab",0,"Dit øje i mit
boret gennem til 
skyld og skam
Vi forstår ikke
hvem vi er,
ansigt dømt til
ansigt
Fremstrakte hænder kan
ikke nå
så længe sandheden 
nægter at
vælge side

Juni 2014

                  "),
("Landskab",621026476032,"Fuglen 
skærer dagen for 
med lange blanke 
lyde
Lyset fanger 
duggens øjne og 
løfter gryet 
ind i 
dagen, 
nyskabt som en
tilgivelse
skænket af 
glæde
Frigivne 
fortsætter vi
vejen vi baner
side om side
lettede for
mørkets byrde


06.11.1013"),
("Landskab",17179877888,"Henstand

Livets mening har 
slået sig ned i
et glas der
bliver til en kalk, hun
hæver med skælvende
hænder og drikker,
til hver celle har
fået besked:
Det går an at 
være her og 
i morgen ligger bag 
bjerge der ikke skal 
bestiges nu



September 2007"),
("Landskab",550210641952,"Hvad er meningen

Fuglene
Kommer ud af vinteren
I farver
Kommer ud af
Kulden
Der viger for
Driften mod
Liv
Hvad er meningen
Hvis ikke længslen
Efter at nå frem
Med olympens fakkel
Løftet gennem
Tomhedens hvide
smerte
At bære lyset frem
gennem mørke
Fuglene leder
under hækken

24.01.2014"),
("Landskab",8640274944,"Landskab

Vilde marker
vokset til med
løse rygter
Overhængende
truslers påstande på den
rådvilde jord
Hvad gør jeg her mellem
vildfarne vækster
Sandheden trækker et
spor over
bakken der skjuler
bagved

Hvis sandhed?

Min  måske et
sted ingen  
har trådt



31. maj 2012"),
("Landskab",0,"Lortevejr

Det regner.
Dagen hænger i en
letsindig tråd over
risten der åbner
en udvej
Mørket har de
bedste kort på
hånden og tager et
stik hjem før
nogen kan
spille ud
Lortevejr hvisler en
læbemund mens
tårekanalen gurgler
taknemmeligt og
lader sig 
løbe over 



April 2015"),
("Landskab",152084496,"Lærkesang

Min krop er 
gammel,min sjæl så 
ung som
lærkens sang der 
spænder evighed fra
jord til himmel

Valmuen er som et
glødende hjerte i
græsset der bare
er


Juli 2015"),
("Landskab",69997724160,"Magt

Smeltede hjerte
løb ikke væk mellem
mine fingre
Hvad ved du om diger
eller havets knusende
kraft


20.nov.2011"),
("Landskab",0,"Mine ord falder ud af
munden og forsvinder i
græsset
Træernes skygger lokker
brændte børn der
samler smuler hvor de
er og giver nyt liv til
ekkoet der allerede
har glemt sin skaber og 
villigt lader sig
bøje i former
ingen har forudset
De knækkede strå
retter sig igen
når duggen falder

25.06.2014"),
("Landskab",134225920,"Månen sår sin sæd i
modne agre
bundet tæt til 
duggens hinde 
Hvad ved vi
om egen vilje 
svøbte i
klodebegær
Som en evig bølge
vokser liv sin
vej,
et lyn, en blodig 
flænge i den 
åbne himmel 


Juni 2004"),
("Landskab",18087936,"Nytårsfortsæt

Han vender
bladet og 
skriver sig på det 
nye års begyndelse,
skriften den 
samme som
sidste år,
bogstaver der
hægter sig sammen og
rækker ud som en 
bro der forsøger at 
nå land



December 2008"),
("Landskab",146028921344,"Nåle

Sorgen er skrevet på
krøllede sedler der
hænger tæt på 
tavlen
Vinduet lukker en
brise ind og lader en  
lap slippe sin
nål og falde i 
cirkler mod
gulvet der intet 
læser men lader 
ordene hvile mod
mørket


29. maj 2011"),
("Landskab",2562,"Overvejelse

Hård vinter og 
lånte fjer er en
risikabel blanding
                  "),
("Landskab",4328816640,"Sommernat

Vinduet svinger på
sine hængsler
Jeg bøjer mig over 
min tanke og
former den
efter min
skabelon

Natsværmeren
svirrer sine
vinger


22. juni 2011


                  "),
("Landskab",152600576,"Søen

Krappe bølger
Fugle har vinger
Flyve, flyve fri af
jeg

En snegl over stien


Oktober 2012"),
("Landskab",135823360,"Truffet

Fuglens sang 
falder fra
træet og hun kan
ikke forklare sin 
indsigt der
fylder hendes længsel med
tro
Hendes skridt bliver
faste og hun går som
én der 
glæder sig
til noget 
forude

marts 2016"),
("Landskab",550846334464,"Udvalgt

Vi står fast i 
hver sin
tro og
har slået rod på
nåden
Vi tåler hinanden fordi 
vi vil og 
har let ved at
tilgive den anden 
højt på
forklarelsens bjerg 
hvor vi ser alting i 
lyset der 
stråler ud fra 
den 
der har fundet 
sandheden i
sig selv og 
løgnen i
den anden


November 2012
                  "),
("Landskab",1207969794,"Vilkår

Sandheden bøjes ikke som
ord

8.maj 2011"),
("Landskab",8758263808,"Ørnen

Kun ørnen ved hvad
ørnen tænker,
lænket til mit
øje
Tænker,
Tænker ikke
Grænseløse længsel
fængsler mit blik
Hvor meget kan én 
enkelt bære
Lad dén svare der
bærer mere end sit
Flyvende skygger over
jorden tegner
fuglens flugt


5. juni 2011

                  "),
("Landskab",0,"Masken

Med en slanges hug
fæstned hun en
maske på hans ansigt –
men først
slog hun spejlet i
stykker -
Han mærkede ingenting
før han slog sig på
et ansigt knyttet i
foragt og fandt sine
veje blokerede af
had
Med tiden frøs han
inde og blev væk i
sig selv og
vidste ikke længere
hvem han var"),
("Landskab",0,"Lim

Jeg tænker i lag
Husker i niveauer
Overfaldes af glimt der
Ikke vil mig godt men
bryder gennem
nuets ramme og splinter
min balance
Billeder
falder på gulvet og
bliver til skår jeg
stædigt forsøger at
stykke sammen til
det der kunne have
været men
ikke vil selvom jeg
forsøger at lime med
de sidste dråber af
længsel"),
("Det ender med et smil",8724676640,"              Høst

            Over de 
          nøgne marker
           svæver en
          rovfugl på
          livets vilje

  det levende løftes gennem
           døden på 
          øjeblikkets 
           mulighed





September 2004

                  "),
("Det ender med et smil",0,"          Glas

    Det smukke glas
       tegner vore
        drømme på
        grænsen af
       virkelighed

           Et 
         enkelt 
        fejltrin
         knuser 
         større
         drømme 
          end 
      sårbart glas



September 2004"),
("Det ender med et smil",2,"Af banen

Jeg må se at få
tiden til at
gå så jeg kan
komme i gang
med det den
spærrer for

September 2004"),
("Det ender med et smil",16777216,"Afgang

En tilfældig
dag
En tilfældig 
time
Et vibrerende
sekund lægger til ved
mit forbehold og
gør klar til 
afgang 
Inget spørgsmål om
destination





Oktober 2004
                  "),
("Det ender med et smil",4,"Aftale med min
krop

Du har båret mig
mere end den
halve vej så
nu er det mig der
skal bære
dig




September 2004
          "),
("Det ender med et smil",512,"Anker

Skyerne driver
forbi selvom jeg
ønsker at
følge med i
det blide lys
Længsel er
intet anker for
skyer der
følger deres
drift uden
tanke på andres 
fortøjning


Oktober 2004


                  "),
("Det ender med et smil",8388608,"Anspændt

Kælderen under
huset lytter efter 
svar i
mørke

En edderkop kravler
over den kalkede
væg. Finder et
hjørne og krøller sig
sammen


Oktober 2004


          "),
("Det ender med et smil",68719476736,"Arkivet

Hænderne finder selv
mappen og sætter dit
ansigt på
plads

Et gulnet minde
opgiver en
halvhjertet
flaksende
flugt


September 2004
          "),
("Det ender med et smil",68728389632,"Bag

Hækken står i skel og
på vagt
Bagved leger 
lydene selv om
en fugl ligger på
æg
Et hul kranser et
øje


September 2004"),
("Det ender med et smil",4563402754,"Bag mørket

Nattens  mørke
stiller spørgsmål
dagens lys ikke vil
kende svaret på



Oktober 2004"),
("Det ender med et smil",0,"Banalt

Det har altid
forekommet mig
mere tillokkende at
afsøge det 
velkendt fortrolige end at
krydse grænser efter
nyt land 
Det aldrig før
sagte skal  nok blive
hørt mens
dagligdagens slidte
fraser stilfærdigt falder ud af
munden uden større
opmærksomhed
Én tager til Kairo i 
14 dage og
vender hjem med færre
impulser end en anden der 
cykler 5 km væk og 
plukker en
buket harekløver på
en eng 
På hjemvejen hører han en
fugl skrige over 
skoven og
forstår pludselig hvad
der er ved at ske 
Og mens den 
elegante ordkunstner
henrykker sit publikum
ser en ældre kone op fra sin
opvask og siger et 
par ord der
får verden til at
vende rigtigt for
sig selv og en 
såret dreng





September 2004"),
("Det ender med et smil",512,"Bobler

Der er  mange 
bobler i dag som
man hele tiden render
ind i og punkterer
Så står man der  med
en   halv 
sommernat og en 
måne det er svært at
kamuflere mens man
forsøger at tale henover  
ord der kommer 
drivende på
tiden
Lidt senere omringes man af
dufte der burde være
blevet hvor 
de var og en
stemme der har 
revet sig løs af 
sin  baggrund og 
bliver ved med at
sige ord det
gør ondt at
høre igen
Det gælder om at
styre udenom og
holde dagen  i  
faste tøjler




Oktober 2004
                  "),
("Det ender med et smil",2,"Boomerang

Man skal vogte sig for
sine ord
De er skabt til at
vende tilbage og
kan komme i en
vinkel man ikke har
forudset


September 2004"),
("Det ender med et smil",7784628224,"Brydning

Den lange nat
spænder sig op mod
dagen og
skubber døgnets
balancepunkt
Der er bitre
kræfter på
spil og ufine
tricks der 
passerer fordi
dommeren er
faldet i
søvn


September 2004"),
("Det ender med et smil",16777218,"Dårligt selskab

Opfatter du dig selv 
som en engel
kommer du til at
færdes mellem
djævle


September 2004
                  "),
("Det ender med et smil",549772632064,"Deponeret

For længe siden 
deponerede jeg min
tryghed hos -
Gud eller hver mand 
- jeg ved ikke længere
hvor
Nu vil jeg have den
tilbage så jeg selv kan
tage mig af den og
finde ud af om vi 
stadig passer til
hinanden og om vi
overhovedet taler
samme sprog



September 2004"),
("Det ender med et smil",558497824804,"Det ender med et
smil

Måske har jeg
grædt for
meget men 
enhver må vel
udstå sín
tåretid -
Og når det nu bliver
tid  at
krydse horisonten at
møde lyset med et
smil og få et
smil igen



Oktober 2004
                  "),
("Det ender med et smil",77309411328,"Dommedag

Jeg venter på
den rigtige dag
Min hensigt ligger
sammenrullet i
øjet og når
tiden kommer
vil det være
tilstrækkeligt med
et enkelt blik



September 2004"),
("Det ender med et smil",68736253952,"Dommeren

Han lytter ind mod
dommen der
langsomt
blotter hovedet og
viser sit
ansigt

Han ser han er
en
af dem




Oktober 2004
                  "),
("Det ender med et smil",68786595904,"Dørene

Der er mange
døre undervejs
og nogen af dem
lukker man når
man går uden at
gøre sig klart at
de aldrig kan 
åbnes igen



September 2004"),
("Det ender med et smil",16777216,"Erkendelse

Et sted ligger en
bog og venter på en
læser
En dag vil den 
åbne sig under
hans hånd  og
lade ham  se et 
svar han først da
erkender at han
har søgt



September 2004"),
("Det ender med et smil",2,"Fatalt

Når grisene skriger af
sult skal der ikke
studeres opskrifter på
kanapeer



September 2004"),
("Det ender med et smil",6710886912,"Flugt

Hun leder efter
lyden af
dag så
hun kan forlade
mørkegrebet og
iføre sig
form og få hold på
det der
næsten undslap
hendes hænder

Så hun kan
gå dagen i
møde som sig
selv og forme
billeder hun
kender

Og vaske mørket
væk



Oktober 2004
                  "),
("Det ender med et smil",16809984,"Fortabt

Et solstrejf 
flytter ham til
centret for
ventende drømme
Den første går af og 
lammer tiden
Uden modgift går
dagen i stå



September 2004"),
("Det ender med et smil",69793251330,"Fronter

Det bitreste er
ikke den åbne
strid men
tågen der
umuliggør enhver
handling


September 2004"),
("Det ender med et smil",137438953984,"Frost

Dagene klædt i
smerte
åbner frosten
ind
Fugleprikkers
afskrift på den
nøgne himmel og
langt væk som
fremmedes
stemmer i
vinden



Oktober 2004
                  "),
("Det ender med et smil",0,"Før

Syv holdninger i
kreds om indædt
tavshed
Nogle nikker svagt
mens alle fylder
stilheden op til dens
bristepunkt


Oktober 2004


Samhørende:
Før - Samling: Det ender med et smil
Nok -  Samling: : Det ender med et smil
Besøget - Samling: Hjertets rytme
Forlegenhedens rum - Samling: Hjertets rytme
Selv - Samling: Det ender med et smil
Dansen - Samling: Kun et øjeblik


                  "),
("Det ender med et smil",549755822112,"Geografi

1. Bag skyerne ligger
himlene hvor 

2. vi har anbragt de
højeste autoriteter 

3. I sikrede  sarkofager hviler
de magtfulde drømme vi
bruger som værn 

4. mod de andre der 
opfylder jorden
sammen med os indtil

5. vi optages i den 
himmel vi har 
valgt

6. Nogle mener, der
findes andre steder 

7. beregnet for dem  
vi ikke kan 
lide




August 2004"),
("Det ender med et smil",550024257536,"Glød

Der er en
glød i
mørket
Et indre skjult
fortætningspunkt
som stjernehobes
forudsætning
Der er et
nærvær i
mørket der
samler det
store øde til
en blidt
omfavnende
form
Og igennem
det store
spørgsmål
strømmer et
løfte om
svar



Oktober 2004


                  "),
("Det ender med et smil",268476930,"Gnist

Mørket holder
tæt så længe 
lyset vil
                  "),
("Det ender med et smil",0,"Høst

De syv søstre skjuler
deres smil mens de
plukker  druer
af  en vildfaren
ranke


Oktober 2004

Samhørende:
Besøget - Hjertets rytme
Forlegenhedens rum - Hjertets rytme
Før - Det ender med et smil



          "),
("Det ender med et smil",86167781376,"Hel

De lange dages lys
løber sammen i
dit smil og 
skjuler sig  bag
glæden i din
latter
Intet er vel
glemt men der er
dannet lyse sprækker
i dine afmagtsår og 
håb der vokser
frem af mørket
som liv der 
stædigt søger
livets  veje



September 2004"),
("Det ender med et smil",8388608,"Hus

Det lukker sig om
sine rum og
står med
blinde øjne

En fugl flyver op i
haven og
lader stilheden
falde væk

Her bor
mennesker der
var glade og hængte
latter og muntre råb op i
havens træer og buske

mens de kransede huset med
duften af
mad og
kærlighedsgaver så
langt lyset nåede frem

og tog hånd om hvor de
kunne

Det brune liv bag
støvede glas på
berøvede vægge

Listende skridt ad
en kældertrappe

En dør der
lukkes med
et undertrykt
smæk


September 2004"),
("Det ender med et smil",8388608,"Huset

Det var opført i
ord
Alle slags sproglige
virkemidler i en
virtuos blanding af
skønne rum
Vægge behængt med
udsøgte citater
Elegante metaforer
spredt overalt
Men man savnede 
stilheden
Spor af et
levet liv og
bare en enkelt
buket selvtænkte
tanker






September 2004"),
("Det ender med et smil",68719476736,"Hygge

Du må hygge dig  med
en kold skulder mens
jeg sidder lårene af 
en anden



Oktober 2004
          "),
("Det ender med et smil",549822930944,"Håb

Det begyndte med
kærlighed

Det ender med
kærlighed

Hadet
går vi
forbi


September 2004"),
("Det ender med et smil",2,"Identifikation

Man kan vist ikke
være sikker på at
et uhyre afslører sig ved
uhyrlig opførsel


August 2004"),
("Det ender med et smil",0,"Kald

Klokkeslag mod
sten
Mures hårde
afvisning
Fald mod
firkantede
flisers spærring. Over byen en
drivende streg 
Græs i sporet efter
dem der drog væk




Oktober 2004
          "),
("Det ender med et smil",68719476736,"Kiggerne

De kigger og
siger ingenting
Øjne gennemkrydser
den rådvilde stilhed men
finder ingen ord at
gemme sig ved
Flakser  fra det
nøgne møde og
afsøger revner og
furer




Oktober 2004
          "),
("Det ender med et smil",68736264194,"Ledelse

Intet system kan
fungere uden
menneskehænder


August 2004"),
("Det ender med et smil",68786585600,"Lykke

Jeg faldt i nat over
dine sko

det eneste sted i
universet du vil
bo


September 2004
                  "),
("Det ender med et smil",68736253952,"Marionet

Klip til et 
billede forskudt for 
sin sammenhæng

løsrevet af
virkeligheden

Der 
falder hun
ind og påføres en
rolle der ikke er
hendes men som 
får hende til at 
agere
Hun ser kun
sig selv 
som 
udefra og
følger rollens
bud uden 
kraft til at 
kigge ud over 
rammen



September 2004
                  "),
("Det ender med et smil",86973090816,"Mesteren

Den ene vælter når
tæppet går op og 
de andre står med
fladtrykte fjæs  og
lader tingene
ske
Selv har han
stjålet deres
stemmer og
dirigerer rundt med
lange pinde  så
alt klapper til
tiden inden
tæppet går 
ned


Oktober 2004

                  "),
("Det ender med et smil",2147483648,"Morgenmøde

Hver morgen holder jeg
møde med 
kroppen der 
påberåber sig sin
alder og det døgn der 
er tikket ind siden 
sidst 
Så giver jeg den en
opsang
Det plejer at
hjælpe



September 2004"),
("Det ender med et smil",68786585600,"Nano

Duften tænder  en 
hukommelse  dybere end 
tanken og i et 
nanosekund 
fordamper tredive år
Selv år er uden
betydning og
der er altid
tid


September 2004
                  "),
("Det ender med et smil",0,"Nok

Stilheden eksploderer og
lader sig ikke
samle

De skamfulde
rester af helhed
ligger

Det nøgne rum 
for tomt til
at bære
enhedens regler


Oktober 2004


Samhørende:
Før - Samling: Det ender med et smil
Nok -  Samling: : Det ender med et smil
Besøget - Samling: Hjertets rytme
Forlegenhedens rum - Samling: Hjertets rytme
Selv - Samling: Det ender med et smil
Dansen - Samling: Kun et øjeblik



                  "),
("Det ender med et smil",68719476736,"Nærhed

Selv om man er
til stede og 
bruger alle
ordene i
lokalet kan
det godt være at
man ikke bliver
bemærket som andet end
en irriterende lyd
der ikke kan
bringes under
kontrol


September 2004"),
("Det ender med et smil",206259093504,"Nøgen

Hvor ofte kan
et hjerte briste uden at
blive til
sten?

Den skære frost
nøgner jorden under
dine skridt og
tegner blomster på
den forladte rude



Oktober 2004
                  "),
("Det ender med et smil",549755822080,"Ophav

Vandet følger sin
bane
styrter ud over og
fortsætter mod sin
forening 
Selv sten lader sig
forme af  den 
viljeløse vilje

Udspringet føder
udløbets ro og
lader liv opstå og
nå frem



September 2004
                  "),
("Det ender med et smil",16785410,"Perle

Som en perles
lag på lag
værner din
erfaring om det 
lille sandkorn der
var dig



September 2004
                  "),
("Det ender med et smil",1140948994,"Pletskud

De sår vi
tilføjes af
vore kære er
de værste
fordi de 
anbringes  med
kenderens
præcision


Oktober 2004

                  "),
("Det ender med et smil",17179872256,"Race

Som en fjern
slægtning af
Kong Gulerod
gjorde han krav på
sine omgivelsers
respekt

Og han var da
også tydeligt
trukket op af
det bed

September 2004
                  "),
("Det ender med et smil",1073752064,"Respekt

Ikke alt er som
det ser ud 
Og selv under
roser kan ligge
gift som en
hævn der venter på
forløsning
Der er ikke
redningsbælter til
alle og de der  har
penge har ikke
råd
Alligevel går det
måske uden
knive I
ærmet

Den ældgammel kone
ligger i afmagt og
folder sine
hænder i respekt for
det der er til




Oktober 2004
                  "),
("Det ender med et smil",67108864,"Ruderegn

Rude mod
regn

Væde der
vil

To dråber til
en fordi 
de er til


September 2004


                  "),
("Det ender med et smil",549822930946,"Rute

Der er mange
biveje men
hovedvejen er
kærlighed



September 2004"),
("Det ender med et smil",68719476736,"Selv


Det er vel ikke
deres sag at
naboerne er
blevet syge
Livet kan 
være brydsomt 
nok endda og
selv syv koste
har nok at
gøre på
eget fortov
Der må være
andre tættere på
andres ulykke og
kun en selv til at
feje for egen dør



Oktober 2004


Samhørende:
Før - Samling: Det ender med et smil
Nok -  Samling: Det ender med et smil
Besøget - Samling: Hjertets rytme
Forlegenhedens rum - Samling: Hjertets rytme
Selv - Samling: Det ender med et smil
Dansen - Samling: Kun et øjeblik



                  "),
("Det ender med et smil",17179870208,"Skudsmål

Det er som at
læse en bog der
kun har den
første side
Måske er resten
ikke skrevet
endnu



Oktober 2004
          "),
("Det ender med et smil",16785408,"Skyer

Skyerne bemærker ikke
deres skygger på
jorden

De er og
lader vinden føre sig
af sted

som det nu er skyers
natur



August 2004"),
("Det ender med et smil",68786593794,"Solstrejf

Vi er her
side om side men
kun i sjældne
glimt tør vi
se hinanden


September 2004"),
("Det ender med et smil",68786585600,"Sorry

Der er kun
en stjerne på
himlen men
natten er lun og
blodet hedt



September 2004"),
("Det ender med et smil",68786585600,"Spadseretur

Vore kroppe går
tur i vort 
fælles rum

Vi er ikke
hjemme



September 2004"),
("Det ender med et smil",16785410,"Spejl

Hvorfor kigge i
spejlet hvis man ikke
vil se?


August 2004


                  "),
("Det ender med et smil",68719476736,"Springflod

Du kæmper med
ordene 
Fra usynlige
kilder flyder de
til
Fylder dig op
flyder over
og floder af sted på
jagt efter 
øregange uden
låg
De fylder huset
Klistrer sig fast til
tapetet
Fosser fra  fordrukne
gardiner og
står knæhøjt i
alle rum
Vi må lukke døre og 
vinduer op for at 
murene ikke skal
revne

Da du falder i
søvn får du 
fred men der
er stadig rester om
munden



September 2004"),
("Det ender med et smil",549756862468,"Sult

Dage er  ikke 
pandekager der bliver 
serveret for en  men
sikke en stabel jeg 
har fået hvis
de var og 
alligevel er jeg ikke
tilfreds men 
håber at ham ved 
komfuret  bliver ved til  
jeg synes jeg er 
mæt og har fået
nok




Oktober 2004
                  "),
("Det ender med et smil",40962,"Synsvinkel

Kun et ur vil
påstå at
sekunder altid er 
lige lange


September 2004"),
("Det ender med et smil",68719476736,"Tabt

Vort udvalgte
minut står tomt og
forsøger rådvildt at
finde mening inden
det går og lader sig
rive med af tiden


Oktober 2004
          "),
("Det ender med et smil",77426851840,"Teater

Rammen er
min og jeg  har
lavet scenografien men
spillet kræver mere
end det, så 
tænder du ikke på 
rollen 
kan tæppet ikke
gå op

Måske er du mere til
improvisation



August 2004"),
("Det ender med et smil",2147483648,"Tegn

Morgenen står
foran og er
ubeskrevet men
bagved knager
spændingerne fra
i går og sender
rystelser gennem den
nøgne flade
Fine revner skriver
tegn det
gælder om at
tyde



Oktober 2004
          "),
("Det ender med et smil",68719478786,"Tidsspilde

Der bliver ingen
debat så længe
parterne står på
hver sin
prædikestol


September 2004
                  "),
("Det ender med et smil",16777216,"Tillid

Lidt har vel
stadig ret så
ganske tryg
rækker jeg begge
arme frem mod
solen



Oktober 2004
          "),
("Det ender med et smil",2,"Trøst

Det gør ondt at
vokse men man
visner i
stilstand


September 2004"),
("Det ender med et smil",8192,"Trofast

Bladene giver slip når
det er tid og
følger deres
bane
mens jeg hænger
fast i  den fælles
forventning:
Man strenger sig
an 
Man bliver på sin
plads


September 2004"),
("Det ender med et smil",6710886528,"Tronskifte

Lyset skærer gennem
mørket og ruller
morgenen ud for den
nye dag der
går os i møde og
lader sig bruge som vi 
vil

Natten afventer sin
tid



August 2004"),
("Det ender med et smil",1073741824,"Tupilak

Først da jeg
opgav at
kæmpe
indså jeg hvem
du var



September 2004
          "),
("Det ender med et smil",269484032,"Udfordring 

Den trætte dag
giver op og
lader bladene
hænge 

De sidste rester af
regn triller
ned og
lader sig suge i
dybet
 
Men nogen 
maler  regnbuen op
på det grå

Kom lad os
danse

tværs over
himmelen


September 2004











                  "),
("Det ender med et smil",0,"Vækst

Først er det 
jomfruelige bogstaver der
umærkeligt finder sammen i 
uskyldige ord 
Inden længe tager de 
hinanden i hånden
og stikker af for at
udforske grammatikken
Siden vender de hjem med
erfarne miner og
før man ved af det
indgår de traktater
stiller op i 
krigserklæringer og 
udraderer fjender med et
pennestrøg



September 2004"),
("Det ender med et smil",69893881856,"Vagten

Du gik men
satte et
ansigt foran min
dør og nu kan
du ikke
komme forbi



September 2004
                  "),
("Det ender med et smil",68736254464,"Varulv

Nattens åbne 
porte lader 
hede understrømme
slippe løs på de
sidste rester af
fornuft

Herren se i
nåde til dem
derude og til
os




September 2004"),
("Det ender med et smil",403709952,"Vægt

Mens forråelsen og
afstumpningen
hoverer 
vokser længslen efter
kærlighed  til et 
flammesværd i
mørket


September 2004


                  "),
("Det ender med et smil",68786585600,"Ædt

Du må have
sanset at
der var guf for
du gik direkte til
min forfængelighed og
åd løs i 
store lunser mens du
drak min
selvstændighed  
Resterne skrabede du
sammen med
øvet hånd  og 
frøs dem ned til en
ret du for længst havde
planlagt du ville 
bruge dem til



September 2004"),
("Klodeskyer",17213424128,"Øen 1


Vogteren


Han bærer øen med sig
overalt
gennem skyggernes
rige

Ser intet

Hører intet

Vogter øens kyster
Årvågent
Aggressivt

Rider utrætteligt
angstens rute

                  "),
("Klodeskyer",17213424128,"Øen 2 

Rytteren


Han rider utrætteligt
langs øens grænser
mod skyggehavet
fast båret af angsten 
gennem skiftende vejrlig
i hans domæne
Skyggernes afstand
som grå gardiner

Slør bag slør

Bag om tågen
ulves gule
seende
øjne

                  "),
("Klodeskyer",17213424128,"Øen 3


Den faste grund


Han taler ikke
for der er ingen
at tale til
Og han ler ikke
for der er ingen at le imod
Og han græder ikke
for der er ingen, der ser
hans smerte

Der er øen
og angsten
Den faste grund
i skyggernes hav
hvor han 
er

Udvalgt
vogter



                  "),
("Klodeskyer",0,"Øjeblikkets mulighed


I stilheds brud
et fugleblik i 
mit i stivnet 
analyse

Overtrådte grænsers
grænseløse mulighed
At se 
At blive set som den 
du er
At måle omfang 
af dit væsen
At møde 
det ukendtes
skabende magt

Flugtens erfarede
nødvendighed
splitter vort
sekund
og den nærhed
der et øjeblik endnu
tøver i mit øje

          "),
("Klodeskyer",17230233600,"Advarsel 

Jeg er bidsk
bider for et godt ord
når du kommer for nær på
min drøm om nærvær
der er større end
vort samvær
kan bære

          "),
("Klodeskyer",8623521792,"Afmægtig 

Vreden har taget mit liv på
og svøbt det om sig
som en overfrakke der er 
alt for lille.
Gennem bristede sømme og
åbentstående knaphuller
vælter hadet ud og 
skolder tilfældigt 
forbipasserende der 
ikke forstår at det er 
umuligt at være svøbt om
et kogende raseri uden 
i det mindste at slippe en 
misstemning løs når 
en ny søm skrider.


                  "),
("Klodeskyer",9714040832,"Bagtalt 

Tågen
har klistret alle udgange til

Selv lyden har den ædt
og gråhvid driver den rundt i dig
ad kanaler
du ikke selv ved af

Kulden lammer dine bevægelser
og du forstår ingenting
Er sat på stand by
af en magt 
du kun aner

Uvishedens
ydmygelse
og angsten for 
den ansigtsløse viden
der måske netop nu
betragter dig bag
tågens sidste slør
har opløst al
substans
og viljeløst
smelter du over i intetheden

Sol og blæst
var aldrig
dine stærke sider


                  "),
("Klodeskyer",1081344,"Ballon


Stemmerne
daler ned over 
bordet som 
krystalkonfetti
en stjernenat,
hvirvles op
igen
af latterkaskader
over klingert
porcelæn
Tidløsheden
omslutter rummet
og svæver bort
som en blå
ballon i mørket
          "),
("Klodeskyer",549891080192,"Ekko


I trøsket træ
en mismodsvinters
omspændte tomrum
stivnet i 
smerteforvansket form
under kronen
under kronen
der som en fakkel
kaster livet
frem fra dybets
flammende
frihedsdrøm

Et levende ekko
af kraften


          "),
("Klodeskyer",0,"Et digt


Et digt
skal være et kys af en fremmed
Et møde hinsides ord
Et velsignende
kærtegn
du bærer med dig
En skænket nærhed
båret af ord

          "),
("Klodeskyer",26877133376,"Fangen


Grænsens virkelighed
står hårdt trukket op 
i det stumme landskab
der lukker sig omkring
hendes tilfrosne ro
Usynlige grænsepæles
forløb afmærker
mulighedernes fængsel
bag vidt åbnede døre
hvor sindets landminer
umuliggør spontan
aktivitet
Agtpågivenheden
snitter skarpe
ridser i hendes 
pande mens hun
lytter efter ekko
af liv


                  "),
("Klodeskyer",206200406016,"Forbi


Skyggespil
på nøgne mure
Skridt
Stilheden bag
blinde ruder
lytter efter
svar på
spørgsmål
der hurtigt
går forbi
          "),
("Klodeskyer",223388664320,"Forrykt


Du drømmer
vågne drømme
kender ikke længere
forskel på drøm og 
virkelighed
- eller er det os
der ikke tåler at 
se dig i 
din sandhed?

                  "),
("Klodeskyer",550963773440,"Frost


Den frosne jord
skriger på
roser i en 
stilstands
nulpunkt
Længslen
søger næring
til bevægelse
i fornægtede
kaosfelter,
hegnet af
ritualers
kontrol
Bedende råb
mod vulkaners
kegler 
          "),
("Klodeskyer",0,"Genfundet


Havets polerede
flade
Tågens sølvslør
i det Krøyerblå
dyb
Køligheden
mod mit bryst
da jeg lydløst
passerer nuet
og tager ophold
i en skær dag
der svæver fri af
levet liv og
lander som
et smykke
i min hånd


          "),
("Klodeskyer",69810028544,"Giv mig lov


Giv mig lov til
at hjælpe dig
Giv mig lov til
at vise dig vej
Giv mig lov til lade mig føle,
jeg behersker situationen

Og giv mig så lov til
at skjule min smerte
Giv mig lov til spille
det spil, vi begge har
gennemskuet
Og giv mig lov til at
vinke med fatning,
når vi om lidt
går hver sin vej


          "),
("Klodeskyer",619550083072,"Grænseløs


Samhørighed 
på tværs af grænser

Samhørighed
på trods af grænser

Folkehavets
råb på fred
skaber
grænseløs samhørighed

                  "),
("Klodeskyer",135266304,"Håndslag


I brudte overfladers
sårbarhed
spirer livets
vilje til udsæd
Overflods dødelige 
mulighed 
Uforbeholdne håndslag
rækkes op over 
betonsmuldrende
øglehud
Ulæste skilte
dingler deres
budskab
over evighedens 
nåde
          "),
("Klodeskyer",85899378688,"Hjemfalden


Du spørger forsigtigt
forsigtigt
for ikke at
vække anstød,
du spørger forsigtigt
om nogen vil hjælpe,
om nogen vil hjælpe med 
den ekstra indsats
det ligesom lå i luften
man gerne så udført i dag
De knyttede rygge fortæller dig uden ord
at du er hjemfalden til timers 
ensomt besvær
fordi du et sted i bagagen
har medbragt en æske
en sirligt indpakket
æske med skyld du 
altid husker
at pakke -
for tilfældets 
skyld


                  "),
("Klodeskyer",17196679680,"Klippen


Som et klippestykke
har du stillet dig i strømmen
der med skumsprøjt
farer dig forbi 
på begge sider
Grå, arret og slidt
Du har lov til at 
slippe din faste fundering
og lade dig strømme med kraften
igennem dit eget forstenede 
jeg og sætte din 
livsglæde fri

                  "),
("Klodeskyer",8591015936,"Klodeskyer


Septemberhimlens skyer står 
til fortolkning mod 
den blå baggrund over bugten
Mit sind har udstrakt sig
hinsides dagen og betragter
roligt deres bevægelse
gennem det rum der 
et øjeblik endnu
er mit

                  "),
("Klodeskyer",550829555712,"Krybben og korset


Barnet
Korset
De tomme hænder

Vi er alene
hjemme med
kærligheden
der kun kan
holdes med
givende hænder



          "),
("Klodeskyer",17179869184,"Lagune


Lyset der toner
dagen
Stilheden
på tværs
af gadens støj
Det rum
du skaber
af nærvær

Lyset
Stilheden
Rummet

Forankret
fylder du
dit omfang
med ro


          "),
("Klodeskyer",552305950720,"Lys


Lyset
Lyset over den endnu
dugvåde dag
Genfødt ånder muligheders
gave begyndelse i 
brudte lag
Afstødte rustningers
natdøde hamme
peger vækstens
vej i lys
På vej
På vej
i lys
mod lys
                  "),
("Klodeskyer",256,"Mosen


Med et gult 
blik
forsvandt katten
ind i min 
erindring
og kom tilbage 
slæbende på en 
lun dag ved 
mosen
En lys drengs
ekko kaldte den
væk til en
drøm
der også var
min under
drivende 
skygger


          "),
("Klodeskyer",16777216,"Omvendt


Alt begynder omvendt
det var ikke dette
jeg havde ventet
men det omvendte
og har svært ved at vende om
vende i strømmen og
finde ind i stimens lov
og følge dens enhed
snarere omvendt
føler jeg
men vender dog om
og følger med som en
omvendt der hemmeligt
drømmer omvendte drømme
          "),
("Klodeskyer",2415919104,"Opstandelse

Vigende mørke
over vandfladen
I spejlet
en lue fra himlens
hjertekammer 
Over horisonten
en skarv i flugt
fra nattens
stilstand,
et styrt mod håbets
Fugl Phønixdød

Stilhed over
den frosne jord

                  "),
("Klodeskyer",33587200,"Regn


Regnen mod
rudens
lyse tærskel
ikke den 
forløsende
opblødning
men cementering
af adskillelsens
grænse


          "),
("Klodeskyer",2147483652,"Rejsen


Uvishedens
slør om
presset af
vækst mod
form
Indlejrede processers
forløb til
planlagte
skibbrud
Fatamorganaers
bølgende læ
for visne blades
dans

Blege spirer sanser
lyset 
længe inden
dag
          "),
("Klodeskyer",1048832,"Sæbebobler

Sæbebobler
i klodeskyer
over et barns latter
Punkterede drømmes
våde spor
som regndryp
på din pande
Forseglede kys
sejler viljeløst
som frø
fra en høstklar
plante


          "),
("Klodeskyer",68719476736,"Skyggevagt


En stemme
taler mellem
korte svar
Dugens mønstre
tegner stilhedens
skygger
på vagt
          "),
("Klodeskyer",69793284096,"Slægtskabet


Der er et syndigt rod i mit slægtskab 
Familiebåndene vælter ud 
og har filtret sig sammen 
i en uoptrævlelig knude 
jeg ikke kan komme løs af 

Jeg kvæles i et net 
af løse ender 
der har viklet sig ind i 
mine hjertebånd 
så det er umuligt 
at få luft til at kæmpe sig fri

Som et sprællende insekt 
sidder jeg fast i klæbrige tråde 
der holder mig på plads 
i et mønster 
jeg ikke vil være del af 

Der skal klippes 
når jeg at tænke


          "),
("Klodeskyer",206426964224,"Sollys 

I et fejende sving slår 
sollyset hul i skylaget 
og blotlægger landskabet 
nedenfor
En vildfaren tanke benytter 
chancen og rækker sig 
gennem årtier ned
til et barn
der engang var mit
Du sulter endnu efter 
et lys jeg ikke gav dig
Må livets sollys finde dig nu


                  "),
("Klodeskyer",0,"Spor

Du står på 
hellig jord
underlagets struktur
skabt af
spor gennem tiden
gemte rester
af levet liv
vævet sammen
af endnu levende 
drømme

          "),
("Klodeskyer",4698701824,"Stjernenat


Stjerner
over lyden af 
dine skridt
Lysende punkter
mod mørket
En fugl bryder
ud af dit bryst
og forsvinder
i lyshavets gåde
Hjertet hviler
trygt
i sin rede


                  "),
("Klodeskyer",81604411904,"Stjerneskud

I nattens mørkeste time
kommer
I til mig
i en hed tyngde
mod mit bryst

I en
ildstorm
mod en sorgfyldt
lagune
under stjerner
der ikke
er mine

Den svage arm
må hugges af
før stjerneskud
slukkes mod
nattens
sugende vande



                  "),
("Klodeskyer",1073741824,"Strategi


Vandstrømmens bevægelse
ikke dens udspring
men retningen
ikke dens mål
men vedholdenhedens
intensitet

en sommerfugls
strandede vilje
          "),
("Klodeskyer",0,"Styrtet

Septemberblå himmel
over gyldne rester
af sommer

en ternes
skrig i mit tomrum
mellem ønske og fornuft
skåret i klippende
styrt gennem
vandfladens
utopi

styrtet fra
håb til vilje



          "),
("Klodeskyer",1048576,"Tur-retur


Jeg sender et smil
tilbage til en 
grå dags 
disharmoni
og hører tonerne
finde sammen
til en enkel 
melodi der får mig 
til at nynne


          "),
("Klodeskyer",215822108672,"Viljers vold


Ild over
horisonten
Flammer
fra en tabt
krig anråber
himlen
Livets forvaltning i
ulevendes hænder
Viljer vil vold

Under udbrændte 
stammer
vokser liljer
frem af
den bundne
jord


                  "),
("Filibuster",0,"Ært

En hård bekymring
triller ud af natten og
falder til ro på
puden
Prinsessen på
ærten forstod sig på
den slags
Gudved hvad der
venter derude



Februar 2005"),
("Filibuster",68854775810,"Ødeland 

Glæde sætter man ikke i
banken
Man bruger så meget
man kan og 
deler resten ud



Januar 2005
                  "),
("Filibuster",68719511552,"Aldrig mere

Det sker alligevel og
alle finder en lomme at
gemme hænder i
eller en dagsorden
der er villig til
at sluge ethvert tilløb til
protest
Øjne studerer
detaljer i et billede der
for en tid får
betydning fordi ingen
kan udholde at
hæve blikket
Vi siger aldrig mere og
mener det




Januar 2005
          "),
("Filibuster",8192,"Bag glasset

Et hudnært derudes
regler
Et enkelt fejltrins
dødelighed 
lige for og
ingen har spurgt 
end ikke om
livets vilje til
liv
At være eller
ikke
på sin side af
glasset




Februar 2005
                  "),
("Filibuster",558765187072,"Bål

Vi er som
gløder i mørket

lysende punkter i en
uudgrundelig plan

uden andet anker end
troen på livet

længslen efter 
kærlighed  og

den fælles erindring om
sammenhæng



Januar 2005



                  "),
("Filibuster",68719509506,"Dørskilt

Kom ikke her med
jeres selvmedlidenhed
Vi har




Januar 2005
                  "),
("Filibuster",8858378240,"Dagene 

Dagene går igennem 
os og afsætter deres 
spor 
Solen står op og 
solen går ned i 
kaskader af foretagsomt lys 
Natten lytter efter 
stemmer i mørket og 
flænges af et 
ugleskrig i 
skoven 




Januar 2005
                  "),
("Filibuster",16777216,"Der var engang

Der var engang og
stadig er vejen vel 
kantet af fristelser der 
venter på den
næste og  mellem de
vejfarende  gemmer sig fortsat de
hjælpere der kender til 
prøver for værdighed

men er prinsen eller
prinsessen  stadig  kampen værd
eller det  halve riges  
gyldne krone

på vej mod et mål  svøbt i
drømme ved 
regnbuens fod






Januar 2005
          "),
("Filibuster",566969245696,"Derfor

Du holdt muligheden
i dine hænder men
brugte den aldrig

Du blev kaldt på men
kom ikke

Dine Ãøne registrerede
verden men du så den
ikke

Du hørte ikke lydene
som ytringer af
liv

Du forstod aldrig at
selvom vi er mange er 
vi én




Februar 2005
                  "),
("Filibuster",268435968,"Drøm

Mørket åbner sig for 
dine drømme og du 
flyver i lys uden 
skygger balancerende ad
knivsægge for at 
plukke blomster  af
de glemtes grave
Du fylder favnen med
tunge buketter og
lader duften svare dit
savn
Under de sovende
træer samler du
smerten af duggen , hastigt 
inden lyset
tørrer den ind
Svøbt i
anelser  bringer du 
høsten i hus



Januar 2005"),
("Filibuster",2,"Eftersyn

Vil man kigge sig
selv i kortene kan
man begynde med at se 
op over sine meninger og
anskuelser  til det
man gør


Februar 2005
                  "),
("Filibuster",0,"Enten eller  

Selvom jeg ikke vil er 
jeg nødt til at gøre det 
Lader jeg være er 
Helvede løs og jeg 
kommer til at 
sidde det af  i 
umindelige tider ,
bliver sat uden for 
døren og set skævt til af 
det gode borgerskab der 
ikke har besvær med 
kalkede facader 

Hvis jeg så bare 
havde tøjet 




Februar 2005
          "),
("Filibuster",402661376,"Fællesskab

Også vi rækker
efter lyset
blomstrer
sætter vore frø

Vi kender til de hårde lag 
vantrivslen i skyggen
kuldens ubøjelighed

Sammen  forkrøbler vi  i 
fjendtlige egne
Slås ned af himlens
vold

De lader sig så men
høster ikke og
har intet andet udtryk for
deres tro
deres håb
deres kærligheden 
end væksten 
blomstringen 

den flygtigste duft




Januar 2004
                  "),
("Filibuster",16778244,"Filibuster

Jeg er ved at være der,
er tæt på at være klar men
har også arbejdet med det,
studeret,
gjort alle de
indledende øvelser
Den karakteristiske gangart
er let i en af dens 
variationer
Tøjet har jeg med undtagelse af 
kasketten
Tænderne  i kategorien
egne går an selvom der
er flere muligheder med 
de andre
De vrissende bemærkninger 
de aggressive blikke
klarer jeg med et 
medfødt talent og
så er der snakken,
de omstændelige udredninger
Beretningerne om fortiden som
jeg har lært udenad for at kunne
gentage dem hvor som helst og
de livskloge betragtninger jeg 
henter fra udvalgte sider på 
nettet
Det sværeste har vist sig at
være den spontane
tiltale af fremmede men det
kommer nok
I går gjorde jeg pungen
klar med et par sedler jeg
krøllede og blandede  med 
en håndfuld mønter





Januar 2005
                  "),
("Filibuster",77376520192,"Flugt

Natten gemt i
søen
Overfladens tætte
spejl
Din hånd  i  
lommen
Glas i dit
blik

Så hør dog dagen der
skriger efter
rovfuglen over skoven




Januar 2005
                  "),
("Filibuster",524288,"Flugt

Tæt over havet
en trækkende fugleflok
Mørkets flygtninge




Januar 2005
                  "),
("Filibuster",0,"Flæsk

Tænk om man kunne
bytte plads med nogen,
gå inde i Dronning Margrethe og
se venligt på folket med 
forgyldt blik og
måske tillade sig at
ændre en smule på rollen og 
læse i avisen  næste dag at 
majestæten havde 
drukket sig fuld til en
middag og skældt en 
ministerfrue ud
Hun skulle vel så være inde i 
mig imens og 
gudved hvad hun ville
tænke ved det for 
hun har vel aldrig spist 
flæsk eller gule ærter og 
drukket en  hel flok bajere til
Måske ville hun blive så 
vild med sofaen bagefter at 
hun nægtede at 
bytte tilbage  og 
det ville vel så betyde  at
jeg blev nødt til at 
uddele et par ordner eller
modtage nogen ambassadører inden
jeg kunne trække mig tilbage til 
hvad det nu er 
en dronning  trækker sig 
tilbage til


Hvad mon hun laver?






Januar 2005
                  "),
("Filibuster",2,"Forståelse

Man kan vælge at
være et andet sted end
kroppen -
og den tager vel også
sine ture




Februar 2005
                  "),
("Filibuster",16777216,"Forsvar

Jeg har pakket det
hele ned i brunt papir
med snor omkring
I dag lukker vi
op
Klipper snore og
lader omslag falde og
alt er der stadig også 
det jeg havde glemt
Noget er anderledes end
jeg husker men
det hele skal
ud og der er
ikke længere frygt i
mit hjerte


Februar 2005"),
("Filibuster",69793218560,"Fra et mellemrum

Det er ikke de
tilstedeværende der
falder i øjnene men
mellemrummene

Oceanerne

Ørknerne

De hårfine korridorer 

Et sugende vakuum

Intensiteten indtil 
øerne har
fundet  form




December 2004
          "),
("Filibuster",636728901632,"Frelst

Hun talte om
frelsen som om
hun tilhÃ¸rte en
eksklusiv art
Og se, hun var virkelig 
en af menneskeracen,
umistelig som hver og
en men det var ikke
det hun talte om og
det var ikke glæden
over livet der fyldte
hendes hjerte




Januar 2005
                  "),
("Filibuster",551099039746,"Fremskridt

Mørkets fremskridt er
skridt mod lyset


Februar 2005"),
("Filibuster",524288,"Fugl

Fuglespor i sneen
En henslængt knipling 
herfra til intet
En flygtig  bemærkning
stemplet i tid



Januar 2005
                  "),
("Filibuster",85899346944,"Gartneren

Hun sår
udvalgte ord i
klargjorte bede

Væksten klarer de
selv



Januar 2004
          "),
("Filibuster",68719476738,"Grænser

Det er som med
stater
Indenfor egne grænser
kan du skabe din
egen lov
men krænker den andres
territorier bliver der
krig




Januar 2005
                  "),
("Filibuster",32768,"Gradbøjning

Det er koldt
derude
men koldere her og
koldest i mit
hjerte

Mit smil vender
nedad og har
tabt målet af sigte




Januar 2005
                  "),
("Filibuster",269615104,"Gyldent

Vægten af himlen
tynger de
sorte træer
Jorden har overgivet sig til
pløren men 
længslen  gennemborer de 
snøftende skyer og
finder vej fra

grå kapsel  til
gyldent hav





Januar 2005
                  "),
("Filibuster",2105344,"Havn

Skibe har brug for
havne
Havet lader sig
bevæge og
søger  sin 
egen ro



Februar 2005"),
("Filibuster",16777216,"Hvem

Jeg ved vel
hvem jeg er

Vinden mættet
af din ild

Månens tårer i
den vidneløse
nat

Under havets
blødende overflade

I lydene af 
kløvermarkens
rigdom

Jeg ved  vel men
leder stadig efter et
spørgsmål der
giver svar

Et blik der 
finder mening



Januar 2005
          "),
("Filibuster",549755855360,"Hvorfor

Der er dagens drømme
og nattens
Der er håbet og
tvivlen langs den
slagne vej
Der er lysten og
pligten,
viljen til blomstring  og
driften mod den spinkle
tros desperation
Angsten der  hudløst
famler ved et hvorfor
Hvorfor skal det lige netop 
være mig 



Februar 2005"),
("Filibuster",16809984,"I form

Jeg er enormt god til at
være sur,
muggen er jo ingen sag, det
har jeg kunnet
længe men  min 
beherskelse af
syrlighed  er nu så 
fuldt udviklet at den  
nærmest ligger udenpå som 
et panser-
i hvert fald hænger 
mundvigene permanent  
for slet ikke at
tale om  øjnene der 
stikker som kanyler når 
de ikke lige er
indstillet på   
gennemsigtig eller 
fjern
Under træningen er det en 
god ide at tage 
udgangspunkt i sin 
sårbarhed og vende den til en
følsom nærtagenhed som 
man siden kan krydre med  
passende grader af
fornærmethed og derved 
efterhånden snige sig ind på
idealet
Med tiden vil man 
kunne spille på
et fuldt register som 
det er vigtigt at
holde ved lige 

For tiden øver jeg mig i
mopset 




December 2004
                  "),
("Filibuster",4295000064,"I parken

Mens natten holder
vejret og lytter til
tunge dryp fra
træet
fanger  dufte et
sølvrefleks og
lægger sig tæt
til mørket

Der er noget på
færde her og det
sviende håb der
krydser stien er
ikke alene




Februar 2005
                  "),
("Filibuster",68920803328,"Indlejret

Mågeskrig og lyden af
blæsten
Havet og solens
reflekser indlejret i blåt
Selv huden husker
vinden og læberne den
svage smag af
salt
Løsrevet som et
lyksaligt varsel om
livets ukuelige kraft og
din hånd der engang
rakte ud efter min





Januar 2004
                  "),
("Filibuster",0,"Indsigt

Brudstykker
tilløb til sammenhænge
i fortegnede detaljer
glemte indsigter og i
et svimlende nu den
ubøjelige sandhed




Februar 2005
          "),
("Filibuster",8192,"Ingen 

Kun du så skovens 
smukkeste blomst men 
ingen så dig 
Ingen målte styrken af 
dit blik og 
ingen hørte dit 
hjertes beslutsomhed 
Ingen så din hånd da 
den trak sig tilbage og 
nøjedes med at 
strejfe et enkelt blad 




Januar 2005
                  "),
("Filibuster",8192,"Kæde 

Og sådan løber tiden som 
en lang kæde uden ende eller 
begyndelse 
Og den løber gennem os 
uden at sætte hastigheden ned eller 
lade sig sinke 
Og den henter sin kraft der 
hvor kraften er og lader 
intet tilbage 



Januar 2005




                  "),
("Filibuster",0,"Kakofoni

Det kan være svært nok at
forstå hvad der foregår
selvom der er ord nok
Flove skilte forsøger at
give mening men har uheld med
besætningen
Sværme af løsrevne
citater svirrer over
hustagene inden de
slår ned og danner
uigennemtrængelige
tekster 
Forvildede ord flakser
omkring og prøver at
slå til




December 2004
                  "),
("Filibuster",68719479808,"Kommunikade

Talestømmens hastighed er
nu så høj at selv den
talendes hjerne står af
med det resultat at
ordene farer  omkring
uden at vide hvor de skal
hen og uden forståelse af
den konstruktion der 
holder dem fast



Januar 2004
                  "),
("Filibuster",33562624,"Kun 

Kun i glimt ser vi 
ud over vore grænser og 
kun i halvmørke pauser 
ser vi scenen blottet for 
kulisser 
Kun i forbifarten ser vi 
de andre som befriet for 
masker 
Kun i anelser 
kommer vi bag på 
os selv 



Januar 2005"),
("Filibuster",50331648,"Længsel

Hun møblerer sin
længsel,
hænger gardiner op
for de søgende
vinduer
Lægger tæppe på
tomheden og
slår sig ned
Den spørgende ramme på
væggen må
vente

Her kan man
leve



Januar 2005
               "),
("Filibuster",0,"Lag

Lag på lag som
bladene i et
træs krone og
langt inde de
bærende elementer der
ikke længere står til at
ændre

Og vinden går over det og
end ikke en anelse lades
tilbage




Januar 2004
          "),
("Filibuster",3074,"Lagdelt

Sandheden ligger
uskadt under
vore ord




Februar 2004
                  "),
("Filibuster",1073741824,"Lavine 

Søvnløs lader natten sig 
rulle sammen for 
fødderne af den kolde 
morgen 

Upåagtede spændinger 
holder endnu bygningen oppe over 
sneskredets mulighed 

Den skarpe sol overser 
fine furers net og 
reflekteres af 
glatte ruders uforstand 

Fredløse jages uden
ophør



Januar 2005"),
("Filibuster",549891088384,"Liv

Det begynder med
liv og ender med
liv
Døden er hvor
intet er 
Selv ting bliver
levende under dit 
blik




Januar 2005
                  "),
("Filibuster",8859688964,"Midvinter

Den kalkede
himmels  låg over  
jorden 

Hvor mange vintre
endnu?

Under hegnet spirer
trodsige vækster med
lyset




Februar 2005
                  "),
("Filibuster",68719476738,"Misundelse

Misundelse bygger oftest på
en fejltænkning
Der går intet fra mig fordi
det går godt  for 
en anden. Og 
ret beset er det vel en
gevinst for alle 
når gruppen tilføres
held



Januar 2005"),
("Filibuster",0,"Nødvendighed

Ligesom små kugler
der ruller gennem 
lange ledninger og
triller ud i modvillige tragter
Ligesom slidte kæder
der slæber over
tiden og lægger sig om
halsen på intetanende
Ligesom en nødvendighed
der ikke vil vide af
hensyn  men insisterer på
en ret ingen har forsynet med
segl eller underskrift
Ligesom en vægt der
tynger det  allerede
tyngede med hele sin 
meningsløshed


Februar 2005"),
("Filibuster",68769816576,"Nøglen

Ord kan åbne
døre
Og låse dem

Nu ved jeg at
jeg kun har
mig selv og at det
slår til



Januar 2005
                  "),
("Filibuster",16777216,"Nytolkning 

Hun har tændt lampen og 
skiller forsigtigt 
billedet ad 
Baggrunden 
de mellemste partier og 
forgrundens detaljer for sig 
Tæt ved kanten finder hun 
en lille person skjult af 
omstændigheder 
Hun bliver lagt for sig selv og 
skal tolkes om 




Januar 2005
                  "),
("Filibuster",68786618368,"Nær dig

Det kunne have været
så godt hvis

vi havde kunnet
nære os men det

kunne vi ikke, sådan 

er det med
os





Januar 2004
                  "),
("Filibuster",68786585600,"Og de levede

Hvis jeg  rækker 
hånden frem
vil du  så 
sætte eventyret i 
gang og
lade spillet
gå til ende, se

det halve riges
krone glimter
mat i morgenlyset



Januar 2005
                  "),
("Filibuster",68736257024,"Og haletudsedag

Uigennemsigtige sump -

men studerer man den
fimrende suppe finder man
de samme typer som 
man kender så godt fra sin
egen andedam




Januar 2005
                  "),
("Filibuster",69793218562,"Opgør

Dommen er hård over det 
man ikke vil forstå




Januar 2005
                  "),
("Filibuster",69793220608,"Ord 

Sådan faldt ordene og 
så var det sagt 

som om de ord kunne
skabe nyt 

som om de  ord  kunne
ændre alt 

ord er vel bare 
lyde vi har tildelt en 
særlig funktion 



Januar 2004"),
("Filibuster",16777220,"På vej

I grunden er man altid
på vej også selvom man 
ingen steder skal hen og
har man sat sig et 
mål er der samtidig mere
på færde end man
er bevidst om
Jamen der bliver ikke
meget tilbage, sagde
frisøren alarmeret, da
jeg bad ham klippe
mig kort, uden at
forstå at det er mit 
rejselook for tiden
sammen med den 
slappe hud og kroppens 
ændrede holdning
Lige nu er der fart over 
feltet 
Jeg er på vej rundt om 
solen og desuden i 
stormløb frem mod en 
rund fødselsdag der
alvorligt truer min 
ungdom
Jeg skulle gerne på
apoteket inden lukketid og
det er jo  ingenting i
forhold til den 
store rejse der  når  som
helst kan bringe mig
ud af syne på vej mod et
mål ingen har
stedfæstet endnu




December 2004
          "),
("Filibuster",0,"Rapport fra
rumvæsen

Advarsel
Planterne har rod
Dyrene territorier
Menneskene forfølger
højere mål i forsøget på at
få magt over noget de
kalder Himmerigs Rige




Februar 2005"),
("Filibuster",16777732,"Rolleskift

Den største del af
tiden har jeg  brugt på at 
forme det billede jeg
troede var ønsket i 
stedet for at frigøre det der
allerede lå parat 
Alligevel er det lykkedes
det at røbe sit omrids og
er jeg heldig  slipper det
i tide løs fra sin kunstige
plageånd så det kan
indtage scenen før
sidste akt




Januar 2005"),
("Filibuster",549772599296,"Søger

Han bladrer rastløst i
tiden  men finder det
ikke
Gulnede sider lader sig
vel skimme men
lukker ikke op
Det må findes  som 
et ekko af løftet på 
første side

En isflages
uimodsigelighed

Mangler der et blad?

Så bare lyden af
et hjerte




Februar 2005"),
("Filibuster",8192,"Scenefolk

Vi ser
Vi hører
Vi føler med
sanser der loyalt
tilpasser verden til
vort indre landskab
som scenearbejdere der
utrætteligt forsøger at synliggøre
instruktørens visioner



Februar 2005"),
("Filibuster",549756862464,"Se

Landskabets skønhed
Evigheden bekræfter 
igen sit løfte



Januar 2005
                  "),
("Filibuster",134217732,"Sent

Ingen tvivl om
din livskraft
men det er let
at se at en anden har
fat i den lange ende



Januar 2005
                  "),
("Filibuster",17196646400,"Silkepapir

Bagefter var det
let at forstå at
det var revnet men før
havde han trygt stolet på
at det var en mur
naturens orden
hele civilisationens
tyngde




Februar 2005
                  "),
("Filibuster",17213424128,"Skygger

Vandring mellem
skygger 
Det grå skyggelands
alléer af levende der
før var mennesker med
rigtige håb og længsler
Hvor glæden og 
forventningen  flød  frit mellem 
åbne favne og          
kærligheden var andet end  
en  antydet bort i
det grå
Sin egen længsel
kender hun ikke men
når hun ser fjernsyn
sker det hendes øjne
løber i vand

Bare ikke se på nogen





Februar 2005
                  "),
("Filibuster",68720033792,"Snegl

Hun var som en
snegl der kravler til
vejrs i den højeste
poppel og flyver af sted på
erfarne vinger
I den døende
aften hører jeg
hende synge fra
nattens første fæstning




Januar 2005
                  "),
("Filibuster",549756870656,"Som om

Som om livet ikke
var nok
Som om det ikke
var nok at være
Som om vi var af
forskellig kvalitet
Som om vi havde
større ret end andre
Som om det ikke
kostede noget
Som om vi var
herrer over
evigheden



Februar 2005"),
("Filibuster",16785408,"Spor 

Vi går fremad i 
det spor vi øjner 
foran os mens vi 
fortolker landskabet 
De der går forrest 
råber bagud til 
hovedfeltet der har 
svært ved at se og 
bagest kommer de mange 
der er henvist til at vende sig 
og se det andre 
for længst har forladt 




Januar 2005
                  "),
("Filibuster",0,"Tanker  

Stilheden fanger de 
gemte tanker 
Griber dem ud af den 
tomme luft og 
lader dem vokse mens de 
svæver rundt i det 
frie rum og lader sig 
se fra alle sider 
Ved den mindste lyd 
er de væk igen som 
punkterede sæbebobler 




Februar 2005
          "),
("Filibuster",16777220,"Tidens vold

Foran spejlet
får man efterhånden
brug for sin
fantasi

Der var engang men
da forstod man det ikke



Januar 2005
          "),
("Filibuster",17179912192,"Tolerance

Så længe det ikke
kommer indenfor
mine døre
Så længe mine
hænder kan blive
i lommen
Så længe det er
ren teori
Så længe  er det
OK med mig




Februar 2005
                  "),
("Filibuster",618475298818,"Tro

En  personlige
tro  på en højere
mening med tilværelsen
kan være en velsignelse
for dig selv og dit liv men
i det øjeblik du
vil gøre den til
mål og rettesnor for
andre bliver den til
en forbandelse



Januar 2005
                  "),
("Filibuster",68787109888,"Trækfugl

I en skrå vinkel fanger
du mit blik og
lænker det i dit
Du er en trækfugl og
ved hvad det
gælder
Om lidt breder du 
vingerne ud og
lader  muligheden 
stå tom




Januar 2005
                  "),
("Filibuster",69860327424,"Ubundet

Der er ingen
regler mellem dig og
mig men det er dig der
bestemmer hvornår vi
skal ses




Januar 2004
                  "),
("Filibuster",0,"Udsættelse

Sneen dækker alt
Et himmelsendt nådelag
der smelter om lidt




Januar 2005
                  "),
("Filibuster",0,"Udtryk

Dette var en ren
side før jeg
sværtede den til med
ord

Hvorfor kan jeg
ikke være stille og
nyde det der er ?

SSSSh




Januar 2005
          "),
("Filibuster",68719478786,"Udvikling

Der findes intet
nyt under
solen
men man kan
da altid flytte
lidt på et
komma



December 2004
                  "),
("Filibuster",8192,"Vækst

Fortiden og fremtiden
fletter fingre gennem
nuet
Veje fører til og
fra gennem land der
lader væksten følge
sin rytme




Januar 2005
                  "),
("Filibuster",566935691332,"Venteværelset

Hun har lagt sin
tro foran døren og
sat sit liv på standby
Hun har iført sig
håbet og forbeholdt sin
kærlighed til hendes tid
kommer ud 
Først da  får hun øje på
skiltet  over døren og
læser teksten: 
Ud



Januar 2004
                  "),
("Filibuster",16785920,"Vilkår

Som et undersøisk skær
afgræsset af niblende
munde
afsøgt af glasklare øjne
beboet 
besat af kæmpende
individer
Som en upåviselig trussel
bag den blå dags
kald en
højderyg under havet
udstrakt i sin egen
nødvendighed
Fiskende fugle langs
bevoksede fjelde
flygtende skygger
i åbne kløfter
fangarme mellem 
opspærrede gab
Som et vilkår under
balancen
En forudsat mulighed for
forlis



Februar 2005"),
("Filibuster",7784628352,"Våbenhvile

Daggryet åbner sig  og 
lader lyset  finde vej 
Mørket kender rutinen og
overgangen er aftale mere
end kamp
I  skyggerkapsler
ligger natten på vagt og
vogter  nidkært balancen



Februar 2005"),
("Filibuster",68820172800,"Ørkenvandring

Der er ørkener
imellem og hvem tror
stadig på oaser
Veje begynder et
sted men hvem
ved  hvor de ender
så de fortsætter 
hånd i hånd  uden at
skæve til siden




Januar 2005
                  "),
("Firs tekster",262144,"Alfehænder

Sommeren hængt frem på
vinterens stativ
Om lidt er det
høst om 
livet vil



4. maj 2004"),
("Firs tekster",16777216,"Ansigter

Sandheden og
løgnen har mange
ansigter
Mon én af de to
kunne nøjes med
mit?


03-01-2004
          "),
("Firs tekster",16779264,"Børn

De står foran
muren med
ørkenens øjne
og ser kun
vejen ad
folkets had

De kender sig
selv i
livets kald


11. maj 2004"),
("Firs tekster",0,"Bag vægge

De høje skræppers
grønne lys
så godt for
øjne der ser 
længere end
skyggevæggenes 
grænse

Den lokkende
verden der aldrig blev
nok fordi han aldrig 
holdt op med at
vokse


23. maj 2004"),
("Firs tekster",68786585600,"Baggrund

Han levede på
hendes baggrund som
han aldrig vendte sig og
så før duftene af
mad forsvandt og
lydende gik i stå


27. april 2004
                  "),
("Firs tekster",32768,"Bare træls

Under indtryk af  de
foreliggende omstændigheder
aflyses alle videre tiltag i
sagen om dine forsvundne
livsmuligheder
Du holdes skadesløs hvad
angår dagens indlysende
fornødenheder men
henvises i øvrigt til
selv at løse de af livets 
problemer der ikke skønnes at 
vedrøre kollektivets ansvar
Ønskes sagen appelleret bedes du 
inden udløbet af tidsfristen rette 
henvendelse til 
overklagemyndighedens første afdeling 
hvis afgørelse ikke 
kan appelleres
Sagen bedes vedlagt alle 
relevante dokumenter i
kopi

01-01-2004


                  "),
("Firs tekster",8725233664,"Basta

Jeg vil ikke
noget og
ikke tage nogen
steder hen
Bare sidde her en
stund og mærke
jeg er mig
Og er jeg heldig
vil jeg høre
universet synge og
lade mine fødder 
tromme rytmen på den 
store runde jord


9. januar 2003"),
("Firs tekster",16785408,"Betingelser

Vi styrer selv men
må følge
vejene gennem 
landskabet og
vejret mens vi
forsøger at komme til
rette med
trafikken og hvad vi
nu ellers støder på


24. april 2004"),
("Firs tekster",139586471936,"Betlehem

Magnoliaens flammer mod det 
uskyldsblå
sinker ikke
menneskets vilje

Morgenduggen røres
vel ikke over
barnegråd


3. maj 2004"),
("Firs tekster",41943044,"Billedet

Huset har
løftet sig ud af
sin ramme og
står kun i hendes
erindring
Dagene mellem de
skæve vægge smutter
hjemløse rundt som
uskarpe glimt på
jagt efter 
mening
Det tågede lys af det
fælles glemte 
breder sig ind 
over fladen og
visker den ren
 


8. maj 2004"),
("Firs tekster",32768,"Bimlegris

I et overset
øjeblik kører en
elevator ned i
et sind og
åbner dørene for
en robust tro på
livets usårlighed
Alt sortsyn bliver
fejet til side sammen med
udtjent håbløshed og
frisk optimisme hængt op på
den slidte dag der 
tager imod som en 
tørlagt dranker og
fylder sig  op med 
100 procents  pur 
lyksalighed uden tanke på 
elevatorer der 
sjældent kun
kører ned



17. maj 2004"),
("Firs tekster",17230200832,"Biperson

Gennem en rids i
tiden så hun spillet bagfra og
opdagede at handlingen ikke
udsprang fra hende men at den
tværtimod drejede sig om 
medspillernes manipulation på
tværs af hendes parti der 
set udefra ikke havde 
antydning af
hovedrolle


8. januar 2004


          "),
("Firs tekster",2048,"Bjerget

Søjlen over
bjerget er
synlig for
enhver
De der tror på
forhandling gør
deres udspil
parat 



16. maj 2004"),
("Firs tekster",17196646400,"Bogen

Til venstre de 
læste blades
afklædthed 
Til højre de
sidste med 
slutningen der
lader sig læse når
den vil

Udenpå 
den tomme plads til en
titel under et 
navn der næppe 
lader sig tyde

Den blev vel 
skrevet af en der 
mente at have
et budskab

1. februar 2004


          "),
("Firs tekster",8589942788,"Brøkregning

På et tidspunkt 
går det op for en at 
selv hvis man er lige så
heldig som man 
plejer at være
har man kun 
cirka halvdelen af 
livet igen
Med tiden bliver 
nævneren større og 
man mister interessen for
yderligere øvelser i 
anvendt matematik


22. maj 2004"),
("Firs tekster",0,"Byen

En forstenet teltby
har slået lejr over 
husene der
holder sammen og
ser ligeud for
ikke at gå i knæ

I lange geledder
kniber de dørene
sammen og lukker
vinduerne til for
ikke at overlistes af
livet på gaden der
på evig jagt efter
ting finder ind i
den mindste sprække der
ikke er dækket til

Skrigende alliker slås på
tagene der ikke tør
blafre i blæsten men
holder facaden for ikke
at se ned på de
vrimlende prikker der
myrer af sted dernede
i ordnede rækker uden
sans for at himlen skriger en 
blodrød stribe i det
lammende grå


17. januar 2004

          "),
("Firs tekster",2048,"Demokrati

Der er den
skjulte magt og
den åbenlyse

For at få adgang til
den skjulte skal du først
få vælgerne til at tro at
det er dem der udøver
den åbenlyse og
at de gør det 
gennem dig


26. april 2004"),
("Firs tekster",16777218,"Destination

De ideer der
flyver af sted på
lånte vinger må
flyve derhen hvor
vingerne vil




17. maj 2004"),
("Firs tekster",0,"Det evige liv

Han svarede mig ved at
lade  rækker af 
ord marchere op ved
min korsvej 

At leve i
øjeblikkets
spontanitet

fulgt af

At lade sig
vugge i
evigheden

og

Lade livet tage sig og
føre med i
universets 
store bevægelse mod
fuldbyrdelsens
egen mening

og længere
fremme

At være

hel

ikke delt

ikke styret af
angstens
mekanismer

Han så på mig og
satte den sidste
deling ind

Hvordan vandre over
Niagara på en
udspændt  line
hvis sindets kontrol
stivner kroppens
leg med
balancen gennem
vandfaldets tågeslør 

over regnbuens
forening med
livet, lysten og
evigheden selv

Jeg havde tabt mit
svar og
gav plads for
hans ord


17. maj 2004







          "),
("Firs tekster",19398656,"Diget

Bag digets længsel
står meget på
spil
Sårbare toner fra
fugle i flugt krydser 
adskillelsen mellem
foran og bagved i
et skærende sekund der
ridser  spor i
utænkeligheden af
svigt

Måske engang men
ikke nu


10. januar 2004





                  "),
("Firs tekster",98304,"En mindefest

I skyldkæder går de
til bords,
samler minderne 
op og
skylle dem ned  med
nuets bitre
eftersmag på
tungen der
et øjeblik finder
ro mens øjnene
forsikrer

Senere går de
væk med hver sin
mæthed og lover at
komme igen




17. maj 2004"),
("Firs tekster",549755813920,"Evigheden

Tidligere forstod jeg
evigheden ud fra
mit liv
Nu er det døden 
jeg tænker på

Måske var jeg bare
visere før


21. januar 2004
                  "),
("Firs tekster",8590000388,"Far

Du var den
eneste far en
lille dreng kunne
få
Du ville aldrig
før nu hvor du
savner sønnens 
støttende hånd


17. maj 2004"),
("Firs tekster",68719542274,"Fjernsyn

Det er dem der
står os nærmest vi
har sværest ved at
få øje på


19. maj 2004"),
("Firs tekster",85932902400,"For fædrelandet

Du kan ikke
forhindre at dine
folde hænder bliver 
våde 

Du vil ikke men 
er blevet
omstændighedernes 
fange

bag stenmasken der
vogter dit 
sind

Måske er det dig
om lidt men 
indtil da skal du 
fylde din plads

menneskebarn


02-01-2004


                  "),
("Firs tekster",135266304,"Forenede styrker

Når man møder sin
egen uduelighed skal
man ikke slå blikke ned og 
ikke  lade sig  sinke men 
fortsætte fremad  arm i 
arm og storme livet
side om side


2. maj 2004"),
("Firs tekster",33554944,"Fortabt

Han talte med
sig selv så
intenst og
vedholdende at
den ydre verden
forsvandt og
han blev et
hul i 
virkeligheden
omgivet af
guirlander af
ord der hastigt
blev trukket ind i
det tomme
rum


30. april 2004"),
("Firs tekster",8623489028,"Fortabt

Udover dagene der
ankommer regelmæssigt
i afmålte intervaller
er der ingen bevægelse i
rummet
Statiske gardiner
hænger impotente
langs udsigten
til et gråt udenfor der
skifter nuance så
umærkeligt som 
årene der trækker livet frem
mod intetheden 
Kun støvet fortæller
historier om
det levende der ikke
mere findes her
selvom det stadig
ånder og føler en
brændende smerte


13. januar 2004

          "),
("Firs tekster",17330864128,"Frit fald 

Da kroppen ikke
længere ville bære
hendes ambition
sprang hun ud over
sine indre dybder

uden Skææærm
          "),
("Firs tekster",16810020,"Fællesrejse

Jeg er i mit
sidste kvarter

Kroppen ved det og
har gjort sig
klar

Forstanden har
kalkuleret og truffet  
sine valg

Men følelsen
er lige ung og
lægger vilde
planer for i
morgen

Vi tre er én
der passer på
hinanden og
går samme vej
som livets vilje
hånd i hånd


10.februar 2004


                  "),
("Firs tekster",279239983104,"Før dag

Kirsebærtræet lader
sine blomster
falde gennem
natten
Den bløde
stilheds
englevinger søger
hjem før
dag som vi


1. maj 2004"),
("Firs tekster",17179870208,"Glansnummer

Hun spillede sit
livs rolle
Var der publikum i
sigte gled fortæppet
prompte op
                  "),
("Firs tekster",77359785474,"Gråd

Selv i de
tungeste stunder er
det os selv vi
begræder


7. maj 2004"),
("Firs tekster",68786585600,"Hans

Det var hans børn
hans hund og
hans rod
Det var hans
gæld 
hans åndssvage
familie og
hans skyld
så han havde 
nok at se til fra
længe inden
natten blev til
dag 




12. maj 2004"),
("Firs tekster",549755879428,"Henkogningsglas


På hylder med
blondepapir

hvilende i
gullige
væsker

stod sommerens
frugter parat i

etiketterede rækker

Tidslommer
foldet om

skeers lyde mod
porcelæn

pisket fløde og
søndagsblå
himmel der

måske og
måske ikke

samler på dem
der  sad med ved
sommerens bord


7. januar 2004





                  "),
("Firs tekster",2097152,"I brændingen 

Lyden af
havets hjerteslag
folder sig og
slutter tæt om sit
foster 

Ånder til
live 

Sletter ud

Skyller kraftfuldt
igennem og river ned til
brugbar grund

Et splintrende opgør
i tiden

Den rensede
flades sarte hud 
kærtegnes blødt af
den nyfødte stilhed


25. maj 2004"),
("Firs tekster",17179869184,"Indflydelse

Du ved hvordan man
handler men
ikke hvordan man
bruger roen
Du kender til at
give ordrer men
kender ikke
lydigheden mod
livet når det 
følger en 
plan du ikke
har lagt


17. maj 2004


          "),
("Firs tekster",68786585600,"Jeg

Han lærte tidligt at
sige jeg  - som
senere blev  til
vi  
men  i grunden 
mente han det 
samme


20. maj 2004"),
("Firs tekster",68769808384,"Kæder

I lange kæder går
vi ind og
krydser hinandens
spor
forsvinder igen og
efterlader et
minde som et
jetflys tågede
streg


17. maj 2004"),
("Firs tekster",524288,"Kald

Bladene folder sig
ud over de kørende
biler der i
strammer linier fastholder
deres rutiner
Fra gadens minaret
dikterer den sorte
fugl sit budskab


6. maj 2004"),
("Firs tekster",17230233600,"Kannibal

Jeg smider en
håndfuld daggamle
bekymringer på
risten og krydrer
med lagrede
ængstelser fra 
skjulte forråd
Kroppen stivner sine
celler under den 
velkendte duft 
der lægger sig om
alt 
emmende af  
løsrevet liv der
ikke mere vil
undvære sin
gift


05-01-04





          "),
("Firs tekster",549755815936,"Karrusellen

Med krængede læber
vuggende forbi
forfulgt af duvende
kaner i
kalejdoskopisk 
repetition af liv 
Brusende hvide
svaners sang om 
frihed fra 
dagenes bundne
kreds omkring det 
lukkede centrum

Den skingre musik holder 
rytmen tæt og
stamper fremad mod
lukkede øjnene og
våde hænder der 
holder fast 

Og over den
snurrende galakse
griner et hvidmalet 
ansigt der hæver 
blikket mod
himlen som en der 
ser væk og
tænker sit


19. februar 2004





                  "),
("Firs tekster",1090552068,"Knapt nok

Han sagde kun et
par ord til et
barn der vel
knapt nok
forstod
Men skyggen var
lang nok til
at dække et
liv der aldrig siden 
tænkte på
sol


17. maj 2004"),
("Firs tekster",1048576,"Kom nu!


Lad os gå ud i
regnen og prøve om
vi kan gå mellem
regndråberne og 
plukke en buket
regnbuer til de
dage der har
opgivet lyset
eller lad os regne
velsignet våde i
himmelsk vand der
måske kan vaske os
rene for alt det vi
gerne ville renses for
hvis vi kendte et
egnet middel
Vade gennem en vandpyt
som en skummende
cruiser og lægge til
ved en ukendt kyst 
der godt kunne være
bredden ved paradis
eller begyndelsen til
et andet liv der ikke
kender til de 
skavanker vi har
lommerne fulde af
Hvad har vi at
miste ud over
drømme der
vel ikke tager skade af
den smule vand


15. januar 2004

                  "),
("Firs tekster",67142144,"Konglomerat

Ondskaben 
drysser ud af 
ærmer der
strækkes frem
i kærlighed
Kys rives op i
sår ingen
kendes ved
Længslens blide
hænder griber
hårdt og
holder ved


17. maj 2004"),
("Firs tekster",549822955520,"Kun der

Urolige hjerte -
forstår du da ikke
den lige vej?


5. maj 2004
                  "),
("Firs tekster",2097152,"Låg

Havfladen kaster alle
spørgsmål tilbage og
holder sit sølvlåg
lukket for
morgenens lys

Timeglasset løber med
samme hast mod
min valne tro


29. april 2004"),
("Firs tekster",8594128896,"Landet

Brunt strækker landet sig 
gennem det blå
indlejret i tiden 
der farer over det i 
skiftende nuancer og 
samlet går de frem
fastholdt i dets 
ro gennem dage de 
stiller til regnskab 
fyldte af længsel efter et 
udspring de har glemt men 
sanser i troens umulighed

Floder og strømme farer gennem dem og 
gennembryder skyggen bag 
skovbrynets første violer der 
dufter sig ind i drømme de 
forsøger at tolke uden at forstå

Fastholdt i rækker synger 
generationer mod den 
bortvendte præst foran 
tidernes alter
uvidende om 
naboens skjulte kval

I pyntede optog drager de gennem 
gader der leder dem frem mod 
mål de tror de har sat

De går gennem tiden ført ad 
veje der peger længere frem end 
nogen kan gå

Lyset splintrer synets grænse over 
landets indfatning i det blå


21. januar 2004


                  "),
("Firs tekster",8388608,"Lineal

De klippede hække i
linier fra 
selvsåede drømme til
ingenmandsland
Mellem husene foldede
tomrum om den
forventede stilhed

En kats regelrette spor
i sneen


24. januar 2004
          "),
("Firs tekster",17716740100,"Løbeturen

Han kravler over
gulvet 
Rejser sig ved
dørkarmen og stolprer
ud i haven hvor han
tumler af sted over 
gangens grus
Lunter langs
haverne mod
parken hvor han 
spæner over de 
klippede plæner  mod 
stien til havet
Strækker spændstigt ud og
løber kraftfuldt 
dynamisk til han 
bliver træt og
finder en  rytme der 
langsomt går
over i gang
Han stempler
sneen med
oldingespor og
ser sig langt tilbage

2. februar 2004



                  "),
("Firs tekster",276824064,"Mørkeøjne

De forklædte vinduer
kaster firkanter ud og 
tegner mønstre på
mørkets afgrundsblik

Langstrakte pletter
springer frem af
intet og lader deres 
månelys illustrere 
flader som efter 
tur slukker sig for
den ventende nat

Mørkeøjne betragter
den tomme stilhed

30. januar 2004


                  "),
("Firs tekster",17179869184,"Magikeren

Han hugger hul i
stilheden
balancer over tiden ad
en udspændt tanke og
vender tilbage med
en spinkel flamme på
sin fremstrakte hånd


17. maj 2004"),
("Firs tekster",32770,"Metode

En kølig vurdering
sætter meget på
plads men
går let glip af 
indsigt der
benytter sig af
andre sanser


17. maj 2004"),
("Firs tekster",0,"Mirakel 

I dag har jeg lyst til 
at være helt ukompliceret og 
bringe ting i orden der 
har haltet længe eller 
ligget og formeret sig i 
dynger 
Udenfor vil jeg se 
mirakelvejr og 
folk på gaden der 
hilser på hinanden med 
glade smil og stopper op og 
fortæller godt nyt fordi 
der ikke findes andet 
Verdenssituationen skal have 
bedret sig på alle punkter og 
når posten kommer er der 
brev fra nogen der bare 
vil det godt 
Det kan blive alle tiders 
dag men jeg opgiver hvis 
det regner  
for der skal mere til 
end mine kræfter 
hvis jeg skal vinde selve 
himlen for 
min sag

25. januar 2004

          "),
("Firs tekster",2147483648,"Morgenkloder


Grene i arret
kontrast mod 
solens beslutsomhed

Æblekloder svøbt i
frost flammet op under
dagens blik

I sneskrift på den
tømte jord 
nattens sterile 
høst


27. januar 2004

                  "),
("Firs tekster",2147483648,"Morgenridt

Med morgenrøde ører
samler jeg dagen op og
spænder den på
ryggen
Den forgange dag var 
ikke tilfreds og om 
hjørnet venter en ny der 
dårligt kan acceptere at 
dagen i dag kommer først
          "),
("Firs tekster",68719477762,"Næe...

Jamen hvorfor dog
være høflig hvis
man har talent for at
være grov?


24. maj 2004"),
("Firs tekster",268435456,"Natklokken

Natten har sat sin
klokke over
jorden og under den 
sorte kuppel står jeg helt 
alene og fylder 
ingenting selvom mine 
tanker var for store til den 
åbne dag og måtte
gemme sig til mørket der
lader dem brede sig og
og springe ud så vi
tilsammen indtager hele
universet 


29. januar 2004

                  "),
("Firs tekster",16777216,"Nok

Der er ingen
genvej til
virkeligheden
Den er og
du er
Det er
nok


10. maj 2004"),
("Firs tekster",2150629376,"Offer

De spiser det
spirende forår og
drikker den lyse 
nat af slanke
glas som de
knuser i
morgenens
brænding

Dagen som
nøgen for
lyd


14. maj 2004"),
("Firs tekster",17179870208,"Oneman show

Tid: Når som helst
Sted: Hvor som helst
Aktører: Ham selv
Manuskript: Ham selv
Instruktion: Ham selv
Publikum: Hvem som helst


20. januar 2004
                  "),
("Firs tekster",68786585600,"Ord

Jeg har brug for
dine ord skønt
jeg selv har så
mange
Du siger ingenting
og skjuler dit
blik
Hjælp mig med
ord 
Jeg behøver kun
et


15. maj 2004"),
("Firs tekster",8589934592,"Overvejelser

Jeg har spekuleret på om
en ræv mon sidder og overvejer
hvorfor den strejfer rundt og
opfører sig ræveagtigt
eller om den bruger tid på at 
sætte spørgsmålstegn ved
rimeligheden af sin livsform
Hvor meget tid afsætter den til
familieliv eller bekymringer for 
de nære relationer
Tænker den på sit
rævelige ansvar for
klodens tilstand og konsekvenserne
af et aktivt ræveliv
Overvejer ræveracen 
hvad det vil sige at
være ræv 
At leve et helt og fuldt
ræveliv for engang med tiden at 
kunne drage til de evige 
jagtmarker mæt af 
dage uden fornemmelsen af at 
være blevet snydt for noget
som andre har 
fået rigeligt af eller
bruger de bare tiden på at
være ræve 
Der må vel være grænser for
hvor lang tid man kan få til at 
gå med det
Og hvad har de mon bag
øret


23. januar 2004

                  "),
("Firs tekster",33554432,"Parallel

De følges gennem
den lyse dag
side om side i
parallelle
forløb

Glaskolbebørn i
drift ad
beregnelige
baners kopi af 
sig selv

Favnet af væggenes
buede ruder
ser de hinanden
glide forbi


3. februar 2004


          "),
("Firs tekster",0,"Poetens bøn

Herre!
Lad det ikke være
smukt for at være
smukt men for at
du kan lade sandheden
spejle sig i det



10. februar 2004
                  "),
("Firs tekster",549755813888,"Pyt er pjat

Hvad tilgivelse er
ved jeg ikke
når det er mig der
skal være den
aktive part

Måske er det ikke en
handling men en
tilstand der nærmest er en 
aktiv form af begrebet 
evighed


26. januar 2004
          "),
("Firs tekster",0,"Rådgivning

Du skal ikke sige det
ingen betydning har
for der eksisterer
sammenhænge ingen
har indsigt i og
årsagsforbindelser der
ligger uden for
vores erfaringsramme
Det har betydning
Vi ved blot ikke hvad
konsekvensen vil blive eller
hvor virkningen vil
vise sig
Det må derfor tilrådes at
du handler efter moden
overvejelse
På den anden side kan man vel sige at
det er ligegyldigt hvad du gør da du
jo som før sagt ikke kan tænke dig til
resultatet af nogen form for handling eller
udeladelse af handling


21. maj 2004


          "),
("Firs tekster",275012124672,"Rødder

Træet viser mig min 
rodfæstethed 

Den forudsigelige
frugt lader sig
plukke af
fremmedes hænder

Kraften vil tænde
nøgne grene  igen og
lade duften ride på
vinden hvorhen 
den vil


19. januar 2003


          "),
("Firs tekster",2,"Rangorden

Simuleret virkelighed måler sig
ikke med virkeliggjort fantasi og
begge overhales indenom af den 
meningsfuldhed dagligdagens 
gøremål kan lægge i ens hånd


14. januar 2004
                  "),
("Firs tekster",17179869184,"Samleren

Han  konserverer
modne tanker og
parate øjeblikke
Sylter fuldbårne sekunder og
skæve vinkler som
han omhyggeligt
sætter på plads i
sit forrådskammer 
med små
etiketter på
rene glas



17. maj 2004"),
("Firs tekster",2097152,"Sekund

Sekunderne samler sig i
minutter der 
stiller sig i kø som
timer
Dagene opsuger dem og
bundter sig i
uger der
formerer måneder og 
år
Århundrederne skriver
historien og afleverer til
årtusindernes klare
overblik men al
handlinger sker i
sekundet
Måske kunne man 
fange et og
kalde det sit


25. april 2004

          "),
("Firs tekster",206242316320,"Single

Så sidder du
der og skal
fylde for
to
Det vante klem
under bordet
smerter


28. april 2004"),
("Firs tekster",68719476740,"Skærmen

Det er ikke de
samme men det er
som har de været her før
Skyggerne dirigeres af
lyset og vinde efter et
ukendt partitur og
vi går med ad en
velkendt tone der
skjuler en ny vibration
De små åbner fremmede 
øjne der ser 
længere end vi og
rækker armene ind bag
skærmen og henter 
mening og sammenhæng
de smider bagud  og 
kræver vi skal
forstå




17. maj 2004"),
("Firs tekster",16809988,"Solist

Hvis mit liv er
en sang er
jeg bange for
den er ude af
takt med tidens
akkompagnement


17. maj 2004
          "),
("Firs tekster",262144,"Sommer

Under de nøgne
altaner
drager sommeren
forbi i 
uantastede dufte
Bag sænkede
gardiner danser
flimrende lysglimt til 
maniske lyde af
mange sprog


9. maj 2004"),
("Firs tekster",68786618368,"Sorg

Du vil jeg
skal tabe mit
ansigt i
bunken af
skårede
chancer

Du sluger
min skyld for at
hele din flængede
stolthed

Vi deler vreden der
ætser sig ind til
åben sorg


17. maj 2004"),
("Firs tekster",68719476736,"Spor

Sporene leder dig
frem gennem mit
landskab

Ad to rækker stål 
spores du gennem 
egne der gemmer de 
felter jeg ønsker du 
skal forstå

I vildnisset synger en
nattergal mens du
forsøger at tørre duggen af
den intetanende rude


6. januar 2004"),
("Firs tekster",549755813888,"Syv

At tage dagen på
sig og fylde den ud

At mødes og lade sine
døre stå åbne

At lade sig opsluge af
sekundets energi

At stå fast på sin
rod og give af sig selv

At modstå bitterheden og
fraskrive sig dommen

At gå ind i den
levende strøm

At møde mørket i
tillid til lyset




13. maj 2004"),
("Firs tekster",8594129152,"Så hjerteligt

Kom
Så kom da ind da
Hvad er du for en
klør
Nå
Hvad ville du sige hvis
du kunne snakke
Stå dog ikke der og kom
fremmed til 
dig selv
Jamen så spyt dog 
ud dreng
Der vel ikke nogen
der har taget din
sukkermad
Var det en lodseddel
eller hvad
Jamen så lad mig da få
en da
Det kunne jo være der
var held ved dig selvom
du ikke kan snakke
Og luk så den dør dreng
Det trækker


04-01-2003


                  "),
("Firs tekster",0,"Træk

Uden genklang glider
dage forbi hvor
intet bærer sig selv
Den lyse latters styrt til
jorden som
splinter der
smerter sig
ind
Et sted på 
et søm en 
dinglende visheds 
kontrakt med 
trækken 
til nogen lukker 
en dør


17. maj 2004"),
("Grib",0,"Ørkenvandring

Ikke en eneste gang
gik det  op for dig at
din stemme var alene
i en ørken af lukkede
gåder

Ikke en eneste gang
forsøgte du at
åbne de låste
svar der lå udstillet i
din talestrøm 

Du bemærkede ikke
solens vilje til at
fremvise din hensigt



Maj 2005
                  "),
("Grib",4,"1 - 0

Med et slag under
bæltet lykkes det
min alder at
besejre min kalender
der må give op og
løslade alle aftaler
Faktisk er der kun én
tilbage som diskret har
lagt sig i skjul bag en
intetanende dato



Maj 2005"),
("Grib",8589943556,"Alleen

Husker du skyggerne i
alleen 
De lange dages
udelte sol  i
flimrende pletter på
sommerbrunet hud
Junglelyset gennem fingrede
blade med glimt af
sagntidens vigende helte
Angsten bag mørkets tærskel
Livet der søgte til og
fra mens det tog og
gav i gådefuld
skæbne
De allerede
falmede portrætter
mellem arrede
stammer,  
stivnede omrids af  fortidens
giganter fanget i
strømmen som
afmægtige støtter
Hviskede brudstykker af
liv  opholdt mellem
grene 
Husker du de inderlige
håb der stadig søger mellem
de levendes svar
 


Marts 2005"),
("Grib",0,"Båndlagt

Der er for langt til
tomhedens kerne

Som en fakkel i
mørket
et hemmeligt håb
holdt I snor



April 2005"),
("Grib",1572864,"Balance

Fuglen svæver på
sekundet og udfordrer
mit angstfyldte håb


Marts 2005"),
("Grib",35072,"Barnetro

Børnene hørte de voksnes gråd og
faldt i rædslen uden grænser
Over tagene voksede de
første stjerner frem af natten og
den stilhed der polerer 
lyde til pur smerte,
ubærlige timer der dog 
måtte bæres af nogen for små til at gradbøje 
angst
Og da morgenen kom i sit
glødende triumftog fandt den mure i små børns
sind og uigennemskinnelige
volde
Båndlagte viljer der ikke
længere greb efter
sommerfuglene ved siden af
vejen men gik for at gå
uden længsel efter
et mål



Marts 2004"),
("Grib",1056772,"Bevægelse

Ikke en bevægelse mod
destruktionen men et 
skridt frem mod
sin egen
fuldbyrdelse,
en frigørende voksen sig
tom for lyst
Ikke forfaldet men en
hæven sig over
begæret, en  laden sig
assimilere af en 
organisk fremvoksende
anelse som et frø der 
vokser sig ud af
sin kerne for at postulere
livets magt



April 2005"),
("Grib",549890072576,"Bi

Tanken modellerer
sit landskab uden på
virkeligheden
Er det overhovedet
muligt at se,
at høre?
Foran mine øjne
invaderer en
bi en blomst og
jeg hører den
flyve summende væk
lastet med mening
Den blanke himmel
favner 



April 2005"),
("Grib",8589936640,"Brodermord

De taler om
brodermord i de
lange gange
Hvisker om
blodspor og
vaskede hænder
Taler
Hvisker
Vasker hænder 
Indtager rutineret
aftalte pladser og
sejler med
lods gennem 
minerede vande
Vinden bærer rygtet fra land men 
ingen hæver blikket mod
horisonten der 
uden lidenskab
spejler alt



Marts 2005"),
("Grib",4294967296,"Bygenat

Under de gamle træer
halter lyden af
viljestærke dråber 

De blå skygger
stiller  skarpt på
nattens stilhed

Lyset fanger sine
pletter og 
trækker dem hjem



April 2005
                  "),
("Grib",268443648,"Cyklus

De lange dage løber
gennem dig og
tvinger dig med

Som solen står op
over den nyfødte
verden
går den ned bag
dagen og
kaster lange skygger
til mørket



Maj 2005"),
("Grib",0,"De andre

De går på gaderne som 
grå skygger
Nogle med øjne
selvom de bor i huse med
blinde vinduer
Nogen taler sammen men
går altid videre
Deres lyde fletter sig
ind i trafikken sammen med
lugtene af deres
liv
Nogle er unge med 
elastiske håb i 
kroppen og er de 
børn sker det de
 hopper af en indre 
kraft der vil ud og
forbi de slidte der kun
brænder svagt 
Her og der ser man de
magnetiske men de
er få og det er som 
solen skinner hvor de
går selvom det
regner på 
alle andre
Om natten er de fleste i
husene
De sidder bag væggene,
under tagene og
lever
Nogle af dem dør  og
andre fødes i
en jævn strøm der
skyller ud i gadernes
sugende morgen





April 2005"),
("Grib",558480490496,"Den sidste  -  den første

Svalerne flyver lavet
gennem det
sidste aftenlys uden
spørgsmål at
kaste i natten
Svævende på den
første solstråle
fanger de livet som det
byder sig til



April 2005"),
("Grib",0,"Den sommer

Middagsheden lader sig
skære i skiver
Landevejen leger stædigt
lineal
Klaser af sommerdage
vælter ud af solen og
falder i klynger
I den lyse nat bliver
længslen ubærlig
Det er nu


Marts 2005"),
("Grib",1342220800,"Det

Det nærmer sig i
uskarpe billeder der
ikke legitimerer deres
indhold eller
hensigt
Det kommer som en
trækvind en lun
aften, et optrækkende
uvejr der måske
driver forbi og
efterlader en
antydet mulighed, som
en toning af et allerede
afstemt billede,
en flygtende skygges
postulat
Det sniger sig ind som en
pludselig lytten der
afsøger æteren for
budskab 
En ophold i lyden, 
en hvisken gennem et 
støjplaget rum



Marts 2005"),
("Grib",2,"Eftertanke

Hvorfor lege med de
lige børn, hvis  de ulige
leger bedre?



Marts 2005"),
("Grib",512,"Englevagt

Lige før hun
falder i søvn
dukker et ansigt
frem af mønstret på
rullegardinet og
minder hende om 
skyggen i kældergulvet 
hun så den dag solen
skiftede farve og huset
lukkede sig

Så er der måske
en udvej



Marts 2005"),
("Grib",2,"Erkendelse

De døre der lader sig
åbne kan være
tillokkende nok men
kender ikke låses leg
med lyst og  nøglemagt



Marts 2005"),
("Grib",2,"Evaluering

Op ad bakke  er også frem



April 2005
                  "),
("Grib",1073743872,"Før

Under isen
Bag bladdækket
Uden stikord i
kulissen
Afventende bag
baggrunden,
skjult under ordenes
uangribelighed
Men når solen
udleverer os til
natten skal mange  se
trods mørket


maj 2005

                  "),
("Grib",1073743872,"Faner

Vi løber gennem gaderne med
hellige faner og råber på 
sejr 
Husene klynger sig til
deres facader og lader
ekkoet hviske om 
vilje til fred


Maj 2005"),
("Grib",33554432,"Fangen

I mørket er øjnene 
fyldt af lys 
Kroppen opretholder
væggenes fald



Marts 2005"),
("Grib",2048,"Forbrødring

Vi sidder ved et bord der
ved hvad det vil  og
inden vi har set os
om langer vi til
fadet og deltager i en
munter fællessang

Kulden udefra prikker kun
på ryggen



April 2005"),
("Grib",268468736,"Fortvivlelse

Natten rullet sammen om
mørket 
samlet til hug
Den stjerneløse himmel
smeltet ind i 
intethedens kerne
Det drømmeløse rum uden
erindring om
lysets mirakel



Marts 2005"),
("Grib",8224,"Frem

Så langt så
godt men
se  ikke tilbage på de
døde der begraver 
deres døde
uden tanke på
dem der gik

Livet vokser  frem
over dødens
gave



Maj 2005"),
("Grib",68719476738,"Fri

Vil et elsket menneske
forlade dig, kan du kun
vise din kærlighed
ved at lade det gå



April 2005
                  "),
("Grib",2,"Frihed

Den frihed du
skænker andre, er
den frihed, du
tilkender dig selv



April 2005"),
("Grib",0,"Gåde

Han er isoleret i 
lyset på den
øde sti  men 
ikke ene
Der er kun ham skønt
han udveksler  med
en anden og
kommer nærmere end
til sig selv

Fuglene flyver
igennem og
fra  det dybe
springer en
kilde 



Maj 2005"),
("Grib",0,"Genfundet

Tæt til vinden
lader jeg tankerne
flyve hvorhen de
vil
En efter en 
flyver de af sted og i
lang tid efter ser jeg
dem rundt omkring og
finder dem genkendeligt
smukke
som lysende indfald i
en gusten dag



Marts 2005"),
("Grib",32,"Gravrøver

Han går mellem
grave i den lyttende
stilhed
Han krydser mellem
urørliges territorier
mens han varsomt
samler lange solglimt
af fremmede navne
Han fletter ranker mod
det hviskende og 
søger svar der kun kan 
besvares af den det 
ikke bekymrer  
Han leder efter
samklang og
føler accept


Marts 2005
                  "),
("Grib",549891088384,"Grib 

Du som bærer
evighedens stempel på
din pande
Du som bærer livets
mulighed
Grib  efter 
dagen, hold den
tæt til dit hjerte og
lad den  mættes ved
rytmen i dets slag



Marts 2005"),
("Grib",1048576,"Gyng-gang

Jeg vil gynge  på en
sang og svinge mig op
over hækkene,
gynge højere og højere og
gribe fuglene i
smilet
Synge mig op gennem
skyerne og sidde lidt
på et ruteflys vinge,
inden jeg svinger tilbage på
en skær tone der
resonerer med  den
lyse dag



April 2005"),
("Grib",0,"Håndslag

Bladende kender
vinden og lader sig
villigt bevæge
Lyset fanger håndens 
vandring og besegler dit 
øjeblik



Marts 2005"),
("Grib",68786618372,"Hjem

Du ser gennem
vore år og indtager
mit blik
Tiden har talt
ud og vi bruger
ingen ord om det
der er blevet for stort til
overvejelse



Maj 2005"),
("Grib",67108866,"Hjælpemiddel

At se med
hjertet svarer til
at tage briller på
ved nærsyn



April 2005"),
("Grib",0,"Hundene

Hvem gider gøre ophævelser over at
hundene gør
De gør!
Larmer ud af halsen for
fulde gardiner 
Hvad enten de er 
glade eller utilfredse med 
et eller andet hakker de
lyde i stykker med spidse
tænder,
bider stilheden over og
spytter den ud i
staccatobidder som de
dropper overalt
Hundene gør mod
himlen bare for at gø
som var det en protest over
noget kun de
kan se

Hvad er der med dem?


April 2005
          "),
("Grib",8192,"I vinden

Helt uden fortrydelse
hælder jeg mig i
vinden og lader mig
blæse ud over det
åbne land der står på
trods og venter 
Hvidtjørnen er parat med
polerede pigge og fra de 
forknytte tuer på 
den sugende eng
hvisker det på et
fremmed sprog der måske
alligevel formidler et
budskab 
i hvert fald en anelse som
lader sig hente 
En spinkel udsæd der 
spirer frem og 
skaber rum,
kælige katedraler og lange
gallerier der ruller ekko mod
angribende lyde
Tomme sale der holder
vejret og frygter
indtrængen 
En forventning om
fuldbyrdelse uden forklaring som
døre der lydløst lukkes op og
muliggør forvandling
Den fyldte himmel skriver 
anagrammer og
lader sig tyde af et
forklaret øjeblik der
flygter fra berøring
Gennem lange skrå skakter
tegner solen udvej for
den frosne jord



Marts 2005"),
("Grib",0,"Indrammet

Rammen lægger sig
tæt om sit
indhold og gør sit til
at helheden tager sig
ud,
strammer op på
uklarhed og lægger
rum til bevægelse
Ingen slingren i
valsen,
de ydre mål tør
lade sig kontrollere af
enhver og med hele
sit virke støtter den
fladens påstand




Marts 2005"),
("Grib",0,"Konferencen


Dette er ikke et digt eller 
nogen anden form for 
tilstræbt, klassificeret 
teksttype. 
Det har intet udadrettet 
formål og har følgelig 
intet budskab, det 
ønsker at lade sig 
aftolke. 
Dette er blot en 
samling bogstaver, der 
er søgt sammen i et 
fælles rum på grund af 
et indre behov. 
Eventuelle beslutninger vil 
ikke blive publiceret, og 
dette vil være 
eneste tegn på, at 
samlingen har fundet sted




April 2005
                  "),
("Grib",33554432,"Kontakt

Ansigter i kantede
udsnit
Indstuderede udtryk på
appellerende masker
Øjne som vandflader i
skiftende belysning og
det dybe vand
Hænder fanget i
bevægelse 
Døre på klem for
åbning eller lukning
Vinduer efterladt på
halv
Lønlige længsler 
uden faste mål
indtil trætheden
sænker blikket og
lader løfterne hænge

Flakkende genskær på
mure



Februar 2005"),
("Grib",17179870208,"Kritikeren

Han er som en
fugl der roder i
civilisationens
losseplads 
Nu og da fisker han en
godbid inden han
flyver væk med et
skrig og efterlader 
en ætsende klat



Maj 2005"),
("Grib",77377601536,"Liv

Hvor er det
levende at være en
anemone,
en lærke der skiller
skyerne med
tvang
Hvor et det liv at
være glød  i 
dine øjne,
hente  varmen i 
din hånd
Hvor er det evighed at
vi er sammen
skønt alt levende
skal dø



April 2005"),
("Grib",558614708224,"Lovsang

Under mørket ligger
den første forsikring om
dag
Bag den blinde
maske stråler lysets
tilsagn
Mørkets magt
rækker kun til
lysets tærskel
Hør den sorte fugls
tillidsfulde lovsang



April 2005"),
("Grib",5368709120,"Luna

Månen kaster sølv over de
åbnede grave
Længslen hvisker i
grønne skygger men
snart skal natten falde for
den  nådeløse dag og
kaste sine grave til



April 2005"),
("Grib",550427983872,"Løberen

Skymassivet over
byen åbner sig  i en
gylden sang  mod
mørket,
en kløft gennem den 
stålsatte himlen
Han kender sin
længsel og lader den
flyve,
bøjer sig ,
snører en sko,
binder sig til
jordens gave


April 2005
                  "),
("Grib",566986055680,"Mindet

Fanget mellem to årstal 
to datoer  husket en tid
Ingen konserverede
omstændigheder
Glæder og sorger hvisket
i vinden

Et navn 

En forsikring om 
savn og  hengivenhed

Forgyldte bogstaver
vejret har slidt


Vi tror du er
mere end det





Marts 2005
                  "),
("Grib",2148007936,"Morgen

Den lyse vind rasler  
poppelens blade og  
vifter solglimt af 
tomme ruder
En gyngende fugl
skærer dagen for 
med slanke lyde



Marts 2005"),
("Grib",0,"Myreby

Huse skåret med
kniv
Lys uden nåde
Som siamesiske tvillinger
navnløse med deres
skygger 
Rastløse myrers
skæbneløb på tid
Lydene bliver ved
jorden som et gråt lag
uden  kanter




April 2005"),
("Grib",549755822080,"Nåde

Vi står mellem stjernerne
og forsøger at løfte jordens
bøn ind i lyset
Vi rækker efter
underet med nøgne
arme og ønsker at
gribe om nådens væsen
Vi sender pile mod
bålet og kaster skygger til
mørket


Marts 2005"),
("Grib",549890039808,"Nøglen

Skyggen fra
træet har fanget et
flimrende net og
jeg tænker at

dette er nok

Ja vel bare et
glimt af den
levende himmel

kan låse nuet op 

nu

                  "),
("Grib",0,"Offer

Lysene brændt ned
en plet på den hvide dug,
her er ofret nok



April 2005"),
("Grib",4294967808,"Pedant

Natten melder pas men
tikker sine ure
Hvert sekund udstanses
nidkært i mørket
Pudens røde sten
hvisker



April 2005"),
("Grib",2,"Police

Vi skal være her
så kort men har så
travlt med det døde
som var det en 
forsikring om 
andel i 
evigheden


April 2005
                  "),
("Grib",8589936640,"Pædagogik

De siger frihed men
mener tvang
De siger lighed men
mener forskel
De siger broderskab men
mener udelukkelse
Og de bliver ved så
længe nogen kan
huske hvad ordene
betød



Marts 2005"),
("Grib",32768,"Rapport

Én ting er at
synke i jorden -
en anden at udholde mørket
kulden
isolationen


Lig begraves
Skatte gemmes
Frø sås





Marts 2005
                  "),
("Grib",103146487808,"Regnbuen

Foran dit grå
maler min palet
regnbuen over
dit hoved

Du ser ikke op



Marts 2005"),
("Grib",2,"Rejseklar

Du kan kun tage
med dig, hvad du kan
bære i dit hjerte



Marts 2005"),
("Grib",549755822080,"Rejsen

Der er begyndelsen og
enden
Der er indgangen og
udgangen forbundet af
en række skridt
Der er glæden og
sorgen
Livets gang gennem
kroppen
Den enkeltes mulighed
Der er fødslens og
dødens gåde og 
overalt 
baggrundens evighed



Maj 2005"),
("Grib",33280,"Rengøring

Jeg må se at få
gjort noget ved det
Irritationen har lagt
sig som en hinde
overalt og gulvene
flyder af utålmodighed og 
henkastede aggressioner
Den dårlige atmosfære har 
farvet gardinerne gule og
i går fandt jeg flere
totter had bag en
indædt radiator

Jeg plejer at klare det med 
en flaske et eller 
andet


April 2005
                  "),
("Grib",0,"Rifter

Månens drageklo
river rifter i himlen,
slipper drømme løs




Marts 2005
          "),
("Grib",68786619392,"Ro

Meditationen
Gentagelsen
Den trygge repetition af
koder
Solen der står op i
samme hjørne på
det forud oplyste tidspunkt og
vandrer over himlen med
automatspeederen låst fast for
at forsvinde bag horisonten til tiden
Hjertets beroligende rytme der
bevæger sig indenfor fornuftens rammer
og de daglige gøremål der
ikke kan opsættes fordi de skal
gentages på samme tid i morgen
Budgettet der holder vognen på vejen
Diæten og lidt under det anbefalede
kvantum sprit
Kønslivet der er fordrevet til sin flaske med
proppen i halsen
Går det vildt til tager vi den frem og
pudser den lidt



Maj 2005"),
("Grib",1048576,"Samklang

Et solglimt i
et blad og
hjertet synger -
Og hvem har sagt at
jeg ikke taler
hjertets sprog



Marts 2005"),
("Grib",77376586752,"Samliv

Der er min
vilje
Og din
Og de skridt vi skal 
gå sammen

Kommer du så



Marts 2005"),
("Grib",2147483776,"Sammenhæng

Dagen rækker hænderne ud
og forsøger at få fat i
kæden men et sted er der
forvirring og en
tid må den stå alene uden 
sammenhæng med fortiden
der er optaget af sit og
uden mulighed for at
danne afsæt for den næste der
allerede forbereder sin
struktur
Omsider skabes der
forbindelse lige netop
i tide til at undg opløsning



Februar 2005"),
("Grib",549756903424,"Sammenhæng

Jeg ser
Jeg hører
Jeg sanser
Jeg husker det
der var og skyggerne
der kun hvisker i 
vinden
Og et øjeblik føler jeg
tillid og solidaritet med
den store organisme og
forsøger at hæve blikket og 
se hen over mørkets tærskel 
Jeg planter vækster der
vil udfordre evigheden og
sår et lille smil der måske
får kraft til uden frygt at 
lade sig smelte ind  i 
solnedgangen



April 2005"),
("Grib",0,"Sigende

Det siger vel sig selv at
man må sige noget hvis
man har noget at sige men
nu og da opstår der faktisk
situationer hvor man siger
mest ved intet at sige



Marts 2005"),
("Grib",2048,"Slut, prut, finale

Løbet er
kørt
Toget er gået så der
bliver ikke mere 
suppe på den 
pølsepind
At stikke fingeren i
jorden er bedre end
ingenting når
geden er
barberet så
tæt at der ikke længere er 
skind at få



April 2005"),
("Grib",0,"Sortie

Silhouetter klippet
ud af natten
svævende i den 
lyse morgens svøb

et øjeblik endnu

som et solstrejfs
kys på din
pande inden din
tid rinder ud

og du fuldender din
skabelse



Maj 2005
          "),
("Grib",135266304,"Splinter

Som et træ 
over mit hoved

fanger hvert blad 
en flig af solen

En grå dag slipper jeg
en byge af
tindrende splinter



Marts 2005"),
("Grib",65794,"Sprængladning

Man kan fastholde sin
forældrerolle så
forbenet, at børnene må
sprænge sig ud af ens hjerte
for at slippe fri



Maj 2005"),
("Grib",68769816576,"Steril

Uden de
andre
Kun i sig
selv på trods af
verdens skønhed


Februar 2005"),
("Grib",0,"Tankeskud

Som et kys
i vinden rammer 
tanken og slår
rod
Spæde skud rejser en svimlende krone
hvor fremmede fugle
slår sig ned  som
syngende ambassadører på  
villige grene



April 2005"),
("Grib",550829557760,"Trælbundet

De tror de har
spærret dig inde i
prægtige katedraler
De tror de har fængslet
dit ord i redigerede
skrifter og 
båndlagt din ånd i 
forstenede ritualer
De tror de har ret til
at dømme og
regulere menneskets
liv
at bruge din kærligheds
tilsagn som magt,
som pression,
som vold



April 2005"),
("Grib",0,"Uddrivelsen

Der står en engel ved
trappens fod
Et flammende sværd i
mørket
Fodspor som stemplet i
blod på den
isnende tavshed
Et sted lyden af 
nogen der græder



Marts 2005"),
("Grib",0,"Udelt

At tabe målet af 
syne og glemme hvor
man skulle hen kan
vise sig at være et  
ringe tab mod 
gevinsten ved  at
blive udleveret til 
nuets  evighed



Marts 2005"),
("Grib",68786586624,"Udsolgt

Jeg ville gerne levere de
ord du ønsker at høre men
har dem ikke på
lager
Hylderne gaber af
tomhed så det bliver det
eneste du får, for du
kan vel ikke tænke dig
andet
Der er tilbud på
sandhed i dag


Marts 2005"),
("Grib",185597952,"Udspring

Blomstrende håb,
forstår du da ikke 
kuldens væsen

Du må vente eller
finde en anden vej 

eller 

skulle det
være muligt at
blomstre sig gennem
isen?



Marts 2005"),
("Grib",1028,"Udvikling

Alle gamle har deltaget i
naturens  obligatoriske
undervisning i visdom.
Den uvægerlige visnen med
tab af kroppens funktioner 
fremtvinger en
stærk indre vækst der  
kunne have  revolutioneret 
menneskehedens  tænkning  
hvis ikke processen  
tillige havde medført en 
tendens til tab af 
vitale åndsevner



April 2005"),
("Grib",67108864,"Usagt 

Som bag de 
flygtende stjerner 
Som bag drivende tågeslør i 
den tidlige morgen 
Under den nøgne jord der 
lader sige dække af 
sneen mens den 
varsomt opbevarer 
betroet liv 
Bag en drivende byges 
skrå gardin
I blæsten der leger 
med toner i påståelige 
master 
Som i dine øjne der 
aldrig har brug for ord



Marts 2005"),
("Grib",4295491584,"Vuggevise

Den sorte bambus
har favnet en spurveflok,
vugger den til ro



April 2005"),
("Halvfjerdserbrun",17179869188,"Øjne


Hendes øjne
har været smukke
engang for længe siden.
Det er de forresten stadig,
men ligesom på en anden måde.
Lidt trætte,
måske knapt så store.
Alvorlige.
De er en gammel kvindes øjne nu.
Mon hun ved det selv?
Jeg ved ikke
hvordan man tænker,
når man er gammel
eller hvordan man føler.
Det må være svært at leve
under de betingelser.
Somme tider
synes jeg
det kan ses
i hendes øjne.



          "),
("Halvfjerdserbrun",68727930880,"140 m2


140 m2 med car-port.
Røde klostersten
og gavle af vandfast krydsfiner.
Haven er de ved at tegne
i haveselskabet.
Den bliver også individuel
og spændende.
Stedsegrøn.
To badeværelser
med lys i de forsænkede
spejle.
8 m2 dansk luksus
til hver af ungerne.
De er brandheldige
at leve netop nu.
Super soft væg til væg.
Oregon pine paneler
og et skønt alrum 
på 30 m2.
Den ideelle ramme om
vort
familieliv.
Vi er sikre på lykken.
Vi betaler for den.



                  "),
("Halvfjerdserbrun",17179869184,"Anskudt


Hun stod stille 
anskudt af et spørgsmål 
der havde ligget 
på lur et 
eller andet sted 
i lang tid 
for at vente på 
en gunstig lejlighed. 
Nu var den der åbenbart 
for hun følte sig 
dødeligt ramt. 
Hvorfor? 
tænkte hun. 
Tanken gjorde hende 
dødelig mat 
og fik hende til 
et øjeblik 
at bide sig fast 
i underlæben 
og knuge vel hårdt 
om det glas 
hun netop var ved 
at sætte i 
opvaskemaskinen. 
Så rakte hun 
beslutsomt hånden 
frem og tændte 
for radioen over 
køleskabet. 
Den virkede. 


          "),
("Halvfjerdserbrun",68719575040,"Besøg


Vi går.
Vi går herfra
inden så forfærdelig længe.
Her er hyggeligt - bevar os.
Særdeles hyggeligt,
men stilheden i de små stuer
fortrænges ikke uden videre
af vore stemmer,
der lidt efter lidt
bliver knapt så sikre.
Nu har vi jo også været her. Spist det uundgåelige franskbrød
med ost.
Drukket af de blå kopper
der for resten er pæne
og vist ikke til at købe mere.
Det er rart at have været her,
men nu må vi nok også
se at komme videre.



          "),
("Halvfjerdserbrun",1245200,"Calendula


Calendula
en krukkefuld
af gule
solskinsøjer
strækker sig
mod vintersolen
tændt i vores
lampe.
Puderne på
sofaen
bag bordets
sorte flade
svarer glad
med samme
glød
et sommerpust
tilbage.


                  "),
("Halvfjerdserbrun",40996,"Dø?

Dø?
Nu?

Jeg troede
jeg var syg
og skulle dø.

Det ord!

Jeg forstod
det 
pludselig 
ikke.

Dø!

Jeg kan 
ikke dø!

Jo.
Ligge i en
hvid seng.
Være meget syg og tapper
men hvad er
dø?

Holde op
at leve
kan jeg ikke?
Min krop
hænder
hud og
hår ved jeg
kan forsvinde
men ikke

jeg!

Og alligevel -
farmor
og alle de
andre
er væk!
Uden dramatik.
Hvor blev verdenspressen af?

Verden kan da
ikke blot
fortsætte
som intet
var hændt
hvis jeg skal
dø!

Jeg forsøgte at
undskylde
mig
for dette noget
jeg ikke
kendte.

Jeg har ikke
tid!
Mine unger er så
små!
- og bedende:
Giv mig i
det mindste tid til
de er store
nok.
Det varer sikkert
ikke længe.
Blot ikke
nu.

Jeg fik udsættelse!
- og forstod 
de gamles afmagt.
Nu hjælper ingen
beden.
Selv ungerne står
klar ved
hullet
med en sidste
blomst
og synes det er
høje tid
de bliver
smerteligt
alene.


                  "),
("Halvfjerdserbrun",68719476756,"Du skal elske din næste


Hende den gamle
inde ved siden af
kom i formiddags
og ringede på døren.
Jeg fik øje på hende
gennem storesen
da hun kom op ad flisegangen.
Hun havde en buket roser
i hånden.
De store gule
hun har i forhaven.
Gud ved hvad hun ville? Det er klogest
at holde sig for sig selv.
Hun kan lige så godt
lære det med det samme.
Desuden har vi heller ikke
tid.
Det er altid de gamle der
er de værste.
De har for meget tid til
at rende andre på dørene
og snage i deres
privatliv.



                  "),
("Halvfjerdserbrun",68719476736,"Hækken


I aftes
da det var blevet mørkt
gik jeg ud til skellet
og gav hækken
en gang gødning.
En ligusterhæk bliver
2 m på 4 år, sagde de
i planteskolen,
og tæt.
Det skulle nødigt vare
længere.
Måske kan det gå hurtigere,

hvis den rigtig får noget
at leve af.
Der er ingen grund til
at naboens skal tro
vi er
uvenlige.



          "),
("Halvfjerdserbrun",68719476736,"Lampen


De havde glemt at trække for.
Gennem hullet i hækken
var der fri udsigt
til etplansstuen.
De havde fået en ny lampe
over spisebordet.
Ikke lige vores smag,
men den gav et godt lys
på fotostaten bagved.
Anemonerne i skovbunden
blev ligesom fremhævet
på en anden måde
end ovre hos os.
Måske skulle vi
flytte spotlightet
over læderstuen.


          "),
("Halvfjerdserbrun",17179869184,"Mulighed

Rådvild
griber du
til dit mest prøvede
nummer:
Profitjagten.

Den slår sjældent
fejl,
eller tvivler du
nu?

Betaler det sig
alligevel
ikke, at lade
det betale sig?

Det er det
der betaler
sig,
der er det
virkelige.

Det,
der skiller
ægte fra
falsk.
Det, der
driver værket.

Eller-

Betaler det sig
alligevel
ikke
at få noget ud
af det?

Er der overhovedet 
noget,
der betaler sig?

Du er smuk
i din rådvildhed
og et let
bytte
for dig
selv.


          "),
("Halvfjerdserbrun",17179869184,"Portræt


Dagdrømmerøjne.
Følsomme hænder.
Et trist lille smil,
der egentlig ikke siger noget,
men kun er signal
fra en hjerne,
der så gerne vil forstå,
men hurtigt bliver træt.
Et sind, der klynker lidt
under presset fra livet,
for det er jo et pres at leve.
Et par ører,
der altid ser lyttende ud,
som hjortens,
når den græssende er gået ud
af skovens tryghed
hele tiden på vagt
efter ting, der måske er farlige
og som det er bedst
at skjule sig for.



          "),
("Halvfjerdserbrun",17179869184,"Portræt 2


Kan det siges så
enkelt:
Hun er ung og
bærer briller
med røgfarvede glas -
eller
Hun er først i trediverne
ganske fikst
klædt på.
Måske lidt kejtet
i sit forhold til
andre.
Levende.
Ordet levende
er virksomt
i denne sammenhæng-
netop virksomt
for hun
har glemt
at også hun
er levende.
Netop 
sitrende
levende kød
der af
en eller anden
grund
holder sig
i live
skønt
hun har glemt
sin egen
identitet
så fuldkomment
at hendes væsen
tilsyneladende
blot
er
røgfarvede glas
i plasticindfatning
Kan det siges så enkelt
Tilsyneladende enkelt.


          "),
("Halvfjerdserbrun",150994960,"Traktose


Jeg går og graver
i min have.

Skaffer plads til nye bede
i den plæne
jeg har gravet op
i sommer.

Den er hård
af død traktose,
jorden
i mit havebed.

Sveden hagler af mig.
Smerten bider alle fibre.

Det er som at hakke
et betongulv op.

Med et går spaden
igennem,
og jeg kan ubesværet
tage endnu et
stik
i en uventet blød
bund.

Jeg er kommet igennem.

I samme øjeblik
forstår jeg,
at jeg for et par dage
siden
brød igennem
et lignende
lag
i mit eget sind
og føler mig helt igennem
hel.

Næste års urter kan
strække tæerne
og hente næring
fra dybe lag,
de har boret
forgæves
efter
indtil nu.

De nye bede
skal aldrig
betrædes.

Aldrig gøres
hårde igen.

Derfor hegner jeg
min have
og laver gange,
man kan færdes ad
og nænsomt række
over
det bløde
voksebed.



                  "),
("Hjertets rytme",559419492352,"12. september


Stadig 11. september, 
og inderst inde 
ved vi godt 
at datoen skifter 
det øjeblik 
vi med kærlighedens kraft 
har mejslet 
12. september 
i vore hjerter. 

                  "),
("Hjertets rytme",558496751684,"Af jord til jord


Af jord er du kommet 
Til jord skal du blive
Af jorden skal du igen 
opstå

Men først skal du følge kroppens veje 
gennem aldre 
der har deres tid 

Du skal lægge dig roligt 
i livets hånd 
og følge en plan 
du ikke har lagt 
men stædigt må vælge 
som din 

Af jord 
til jord 
Og hver dag skal du igen opstå 
fordi
livet vil det


                  "),
("Hjertets rytme",671358980,"At løbe


At løbe gennem skoven
gennem efteråret.
At løbe gennem
udlevede landskaber
der forbereder deres vinter.

At løbe gennem tiden
gennem aldersfaser.
At løbe gennem
fuldbyrdede tidsaldre
der indeholder næste forår.

At løbe gennem livet
gennem udfordringer.
At løbe gennem
udspillede epoker
der forudsiger stadig sommer.

At løbe sin rute til ende. 


                  "),
("Hjertets rytme",68727865344,"Besøget


De syv søstre har inviteret sig selv
indenfor og sidder og ryger i
vore gode stole,
så røgen skjuler os
for os selv.
De kigger lige forbi,
mens vi giver
en drink og tager sig tid til
at farve vore gardiner med
jeremiader om deres liv.
De kaster øjne rundt i stuen,
hvor de maler væggene fulde af projekter,
vi burde hjælpe med.
Takserer vort bohave,
prise det problemløse liv,
som skæbnen har tildelt netop os
og fra syv lange snabler suger
de blodet af vore årer
for at lade os ligge som slatne balloner,
da de med smilende råb tager af sted
fra dette sjældent rolige og 
harmoniske sted.





Samhørende:
Før - Samling: Det ender med et smil
Nok -  Samling: : Det ender med et smil
Besøget - Samling: Hjertets rytme
Forlegenhedens rum - Samling: Hjertets rytme
Selv - Samling: Det ender med et smil
Dansen - Samling: Kun et øjeblik





                  "),
("Hjertets rytme",18253652032,"Drageporten


Klodens nyfødte nervesystem
rapporterer lidenskabsløst 
min død af AIDS 
på en sæk i et primitivt hospital 
i Afrika
Fluesværmens summen overdøver 
mine børns skrig da monsunen 
oversvømmer vores egn og får husene 
til at falde sammen
Overalt ser man blodet strømme 
fra mine fingre der syr fodbolde 
jeg aldrig skal spille med.
Man hører min demente talestrøm 
og iagttager i nærbillede mine 
forbrændinger, mens jeg bygger 
en hytte af affald på en storbys losseplads

Som en ildspyende drage 
står klodebevidstheden 
foran paradiset 
hvor de ventende masser 
forklæder sig i 
kynisme

Se mig
Hør mig
Tal med mig

Jeg udstiller mig skamløst 
Søger døden ved
egen hånd 
stivnet foran 
drageporten


                  "),
("Hjertets rytme",549755822080,"Enhed

Helheden rummer
detaljen og lader sit
omfang spejle 

Kærlighedslængslen
drømmer den færdige
form og stempler
uendelighedens tegn
på din pande

Over smertens vækst
lyser enheden
mørket til ende



                  "),
("Hjertets rytme",8589934592,"Fire spørgsmål til 
blind makker

Mon der er en
mening med den 
galskab der
vælter drømme og
knuser dem under 
dynger af sorg

Stiller vægtarmes ubalance
krav om ligevægt

Hvad vejer en klodes 
forenede længsler 

Hvornår er der 
nok


          "),
("Hjertets rytme",549890039808,"Forførelse

Drømme om skønhed,
om enkelhed,
sammenhæng

Sandhed
Renhed
Kærlighed

Frelse

Søndrede verden,

vi følges ad
på øde stier
ingen betræder
alene

                  "),
("Hjertets rytme",558481022976,"Forjættelse

Fløjteblide vårnat
Stadigvæk 
hjertets lyde 
mod evighedens 
stilhed
Vægtløshedens stjerner 
altid på vej mod 
dugfrisk liv 
vi ikke 
behersker
                  "),
("Hjertets rytme",68727898112,"Forlegenhedens rum


I forlegenhedens rum
sidder vi til bords
med de 7 søstre
der ved alt, der er at sige
om materielle og immaterielle emner.

I forlegenhedens rum krummer vi tæer i skoene under borde
der knapt nok dækker vores skam.
I forlegenhedens rum
forlegenhedens trynen af vore bedste sider.

Vi sidder til bords for længe.
Sidder for længe til bords i forlegenhedens rum
med de 7 søstre, der smiler med tandløse gummer.









Samhørende:
Før - Samling: Det ender med et smil
Nok -  Samling: : Det ender med et smil
Besøget - Samling: Hjertets rytme
Forlegenhedens rum - Samling: Hjertets rytme
Selv - Samling: Det ender med et smil
Dansen - Samling: Kun et øjeblik









          "),
("Hjertets rytme",549890039808,"Hjertets rytme


Kraftfuldt fødes 
organiske former 
ind til livets 
vilje til 
vækst 

Bevidste former som ubevidste 
kæmpende
for at bevare det egne selv
og række det ind 
i tidens 
evige nu 

Livets væren 
i tid og form 

Et skrig efter menig
fra bevidstheds 
villen 
forstå

En kamp om resourcer
livsrum
og magt
Om retten til livet 
der føder konstant 

Det værende
strømmende liv 
gennem alt





                  "),
("Hjertets rytme",136314880,"Kraften

Havets silkehud bølger over 
hvalernes  rejse mellem
kontinenter i drift

Magtfuldt strømmer kraftens 
årer udspændt  mellem 
fladers opstemmede ro 

Modne sneskreds
klarsyn  styrter
mod  balance

Magtens vej
aflæses i blade der
danser på vinden



          "),
("Hjertets rytme",550176301056,"Lysbærer



At være sin egen
under solens lys i 
dugdråbelandet og 
under de 
isnende stjerner
At være sin egen i 
skyggelandet og 
bære livet med sig som 
en skænket juvel
fæstnet på sin 
pande


                  "),
("Hjertets rytme",0,"Mønstre

Hvem er du der
venter i
skyggen
for foden af
dagen

Hvem er du der
gemmer dit
ansigt bag
ordenes slør

Ingen besvarer et
spørgsmål der
ikke kan
stilles

Støvet dækker
slidte mønstre
ingen ser


          "),
("Hjertets rytme",553379692546,"Magt

Det lys der
udfordrer
mørket har
større magt
end mørkets
stærkeste
vilje
                  "),
("Hjertets rytme",549891080196,"Modning

Grønne marker
kender ikke til
gyldne kerner

Et ord er
nok når
høsten
vil
          "),
("Hjertets rytme",402661376,"Nat


Lyse månenats 
løfter 
Stilhedens lyde i 
skyggerelief 

Lysende 
måneblide nat 
Kloder i rolig drift 
ad forudlagte baner 
mod fuldbyrdelse 

Månelyse nats 
leg 
med anelsers 
indsigt 
om sammenhænge 
vibrerende af liv 
gennem skabelsen


                  "),
("Hjertets rytme",134217728,"Plaster

Levende side
om side
fra intet til
intet

livet farer
gennem ler

Gennem kæmpende 
kroppes afmægtighed

Snu beregningers
kassebøger bygger
paladser på
skyldneres
fundamenter og
limer sten på sten med

tilgivelsens overbærende
plaster


          "),
("Hjertets rytme",549890039812,"Resonans


Spiralens
forløb
gennem tiden

Spiralens
forløb lysende 
over døde byers
udbrændte smerter

I det øde rum
en søgende tone
nynnende sig ind i
det levende lys
for enden af trappen



                  "),
("Hjertets rytme",558481154064,"Solsikkedans


I kreds af nikkende hoveder
flammer gyldne længsler
mod det blå dyb, der som
havet bærer liv i svøb
gennem tiden

En duvende solsikkeklodes
svangre dans

                  "),
("Hjertets rytme",549891342336,"Solskin

Solskin over
nøgne marker der
samler endnu en 
udløsning af 
kraften der
rytmisk strømmer
gennem det
døde ler

Solskin over
bankende hjerter
der pumper på
trods og forkynder
livets nyfødte
mulighed


                  "),
("Hjertets rytme",592839704576,"Tegn

Forårets lys
strækker regnbuens
symfoni
over det svangre 
landskab

Usynlig vækst
markerer
skaberkraftens
vilje

Tilgivelsens
nådebro
forener
det aldrig
adskilte

Lysets skære
tegn


                  "),
("Hjertets rytme",824767946756,"Under


De faldende
blade,
den beredte
jord
opblødt parat
til næste
stade, hvor
livet under 
dødens magt
vokser uafbrudt 
tilbage



                  "),
("Hjertets slag",6,"Aldring

Nogle bliver yngre med
årene og genfinder
barndommens visdom


Juli 2005"),
("Hjertets slag",137539649536,"Alene

Til sidst er der ikke
mere at tale om,
ord gør kun
ondt ubærligt
En hånd på din skulder
rækker dig
afmagtens tomhed


Juni 2005

                  "),
("Hjertets slag",17179870208,"Bæreren

Hvis ord var
nok bar han
nøglen til paradis i
munden


Maj 2005"),
("Hjertets slag",134348816,"Blå

Himlen på jorden 
under min
hånd,

mursprækkens
selvbudne vækst ,

tankespringets
sprængende styrke


Juni 2005"),
("Hjertets slag",85899346944,"Brugt

Hun er ikke sit
sædvanlige selv,
nærmest som en
pose nogen har tømt for 
bolcher så der kun er lidt 
kulørt krymmel tilbage 
på bunden



Juli 2005"),
("Hjertets slag",0,"Camouflage

Himlen er stribet som
et stykke papir der
venter på en
hånd med
et vigtigt budskab,
en kort besked
eller bare et par
stikord til at 
camouflere den
nøgne tomhed



Juli 2005"),
("Hjertets slag",134225920,"Den unge vin

Fra  indkapslet
hvisken klækkes 
foråret i
et svulmende brøl

Den der ikke kan bære 
endnu et  udspring må 
fuldbyrde den
organiske  vej

Over de  blottede grave
stiller den nyfødte sine
krav  som  lovsang mod
himlen 

Den blinde lov  
bærer skabningen frem på
ubøjelige paragraffer 


Juni 2005"),
("Hjertets slag",631444078592,"Du var

Du var træet
Jeg var skyggen
Du var natten 
Jeg var stilheden mellem
mørke grene
Du var hjertet
skåret i barken
Du var hjertet jeg
bar i mit bryst



Juli 2005"),
("Hjertets slag",8607236096,"Due

På kanten af
bevidstheden
svævende mod jorden
med dens besættende
længsel

I en afkrog af mit væsen ved jeg
hvordan det er at hæve sig over jorden og
give sig hen til luften,
udmatte musklerne under en stejl
stigning og svæve ned på højt løftede
vinger for at lette igen og atter forvandle
jorden til farvede felter og
klodser
Jeg kender til den hastige flugt over
havet, den salte duft blandet med årvågenhed,
flugten fra den forfølgende rovfugl,
kampen,
døden mellem knugende dolke og
lyset der eksploderer i liv
Jeg kender til det og fylder årerne med
kurren der genlyder fra mit eget bryst,
vogter horisonten i den forvandlede
hvælving,
lyslevende i en trækkende flok
Den kollektive vilje
frihed fra længslen på jorden,
overgivelsen til nuet og
den fælles krop




Juni 2005"),
("Hjertets slag",2,"Ekko

Fortiden kaster ekko mod
nutiden der kan have
svært ved at skelne 
egne tanker


Juni 2005"),
("Hjertets slag",279441309696,"Flammebrøl

Poplen hævder sin
aftale med vinden og
lader sin raslen
stige til  brøl over
mørkets forsegling af
huse

Morgenlysets fravær skaber rum 

Som en fakkel mod
himlen
nattens flammende
hoveren


Juni 2005"),
("Hjertets slag",268435456,"Foran muren

De lange dage kaster skygger
på den tomme mur
Foran en solsorts budskab
fanger  solen  sommerfugle og
lader dem skrive  i det
flimrende guld



Maj 2005"),
("Hjertets slag",4294967296,"Foran porten

Sangerinden i den røde
kjole venter foran den
lukkede port
Snart galer den
tredje hane og 
fuldbyrder  nattens
beslutning



Maj 2005"),
("Hjertets slag",69793219584,"Fra mig
Til dig

Ord pakket ind i
ord med en 
letfordøjelig
sløjfe



Juli 2005"),
("Hjertets slag",137472507936,"Grænseovergang

Fra din smerter
ser du hen over håbet og
i dine øjne læser vi 
viljen til at krydse
grænsen der
holder os tilbage

Vore hænders tomme
vilje



Juni 2005"),
("Hjertets slag",551233259520,"Håb

De kan slå ihjel men
de kan ikke
dræbe livet

De kan ætse
vores mod med
had men aldrig
fjerne længslen efter
kærlighed


Juli 2005"),
("Hjertets slag",2,"Handlekraft

Man kan komme i
tvivl om det er
os der tager
beslutningerne eller
det er beslutningerne der
tager os



Juli 2005"),
("Hjertets slag",150994944,"Her

Landskabet løber
gennem mig og afsætter
sine spor,
toner mine farver og
bygger rum,
udspænder omfang og
lader sine skyer drive gennem
mine drømme

Vinternøgne hegn søger
forløsning i blomstringens
vildskab
åbner sig for vindens vilje
og lader sig forme

Amorfe strukturer 
bearbejdes i
undergrunden af
ubøjelige kræfter

Den evigt foranderlige
himmel afspejler
havets tema og
tænder mine øjnes lys


Juli 2005"),
("Hjertets slag",73081552896,"Hinsides

Vi hænger vore
løfter på
månens horn og
lader os flyde på
nattens dufte 
Den ædru dag
forliser bag mørket
der hverken kræver 
pas eller anker 
Så levende leger
livet med os nu som
for evigt  og
over os hører vi 
lyden af
bløde vinger


Juli 2005"),
("Hjertets slag",0,"Hjemkomst

Intet spor af
stjernerne og dog
er de her 
lysende over 
mit hoved

På lydløse vinger
vender fuglen  tilbage til
reden


Juni 2005"),
("Hjertets slag",77410074624,"Hjertemur

Der er en ring om
månen i nat og
mure omkring mit
hjerte
Syv dage gik de
om Jeriko før
murene faldt for deres
trompeter
Syv gange for
Jeriko men
hvordan åbner man sit
hjerte



Juli 2005"),
("Hjertets slag",8590983168,"Hjertets slag

Jeg lægger mig på 
maven og lytter efter 
klodens hjerteslag
Fra dybet høres livets
lyde som universets
åndedrag og under det -
det store hjertes løfte 
om  en dag endnu
Og jeg føler kraften pumpe i
min  krop der
klynger sig til 
jordens tryghed
for ikke at falde op og 
blive en af
himlens stjerne




Maj 2005"),
("Hjertets slag",279308140544,"Hyld

Sommernatten henslængt
langs den
lysende fjord

Gyldne drenges latter
skærer stilheden for 
ved nattens duftende
månetræf


Juni 2004
                  "),
("Hjertets slag",627804504101,"I kærlighed

Jeg vil løbe som en
dreng over de
høstede marker,
sidde øverst på
bakken og være
ældre end min
alder, se på
livet på afstand og
prøve at forstå
Indsnuse duften af
det spæde forår,
svømme i luften som det
grønne inden det slår sig
ned i bøgens lysende
flimmer
Jeg vil endnu engang gå 
gennem barndomslandet og
lyttet til eventyret der
hvisker i hyldens skygge
ved stranden der var engang
Jeg vil tage din hånd og
vide at afstand aldrig kan
skille det der er 
føjet sammen 
før det af hjertet 
ønsker at gå



Maj 2005"),
("Hjertets slag",549806145538,"Isoleret

Vælger du Gud
på bekostning af 
mennesker,
vælger du 
meningen med livet 
fra



Juni 2005"),
("Hjertets slag",558497792000,"Jorden

Jeg vil og
jorden føjer sig under
min krop,
blottet, viljeløs,
åben
og det er klodens kraft
jeg bruger
Jordens velsignede
gave til en
skabning der har
rejst sig af mulden og
triumferer inden den
vender tilbage igen


Juni 2005"),
("Hjertets slag",134217744,"Kamp

Græsset føjer sig efter
vinden. Kæmpende 
for lyset 
modnes  frøets 
mulighed

En spalte i tiden

De nøgne sletter åbne for
himmel



Maj 2005
                  "),
("Hjertets slag",68719509504,"Kløften

Først da din
afmagt rækker
over kløften
forstår jeg din
styrke og vi træder
begge frem hvor
der før var tomt

Nøgne for intethed
finder vi hinandens
hænder


Juni 2005"),
("Hjertets slag",33652996,"Længsel

Du bærer en
byrde barn
jeg ikke kan
løfte

Min afmagt er som en
ørken der også skiller os fra  

os selv



Maj 2005"),
("Hjertets slag",4294967296,"Linedans

De kommer ud af 
skumringen som 
fortættede flager af 
nat
På stumme skrig afsøger de
rummet mellem
byens tårne der 
står på tå for at
fange det sidste lys

Neonlysende erindringer 
flimrer over  de
sovende facader uden 
hensyn til nattens fortrop

I bratte klip sakses  mellem
de duftende lindes 
liner udspændt fra
før til nu

Så let
så let at 
balancere gennem 
denne spæde nat







Juli 2005"),
("Hjertets slag",1048578,"Mål

Ser du dig omkring
vil du få øje på
flere målskiver end
du havde troet
muligt
Og der er tid til at
vælge en ny




Maj 2005"),
("Hjertets slag",17315135492,"Måske

Hun har gået  med
burde så længe at
hun har fået lyst til
at gå alene og
lade sin skygge
falde hvor den vil
uden at skulle tænke på
andet end dagens
skønhed
Ja måske vil hun
vælge en helt
anden vej og plukke
en stor buket
glæde mens der
endnu er tid



Maj 2005"),
("Hjertets slag",69810063362,"Maskepi

Problemet er ikke
så meget de
masker vi skjuler os
bag som dem vi
klistrer på hinanden



Juni 2005
                  "),
("Hjertets slag",2,"Med fast og kærlig hånd

Du skal være din
krops bedste ven men
altid have  pisken klar
bag ryggen


Juni 2005"),
("Hjertets slag",549890031616,"Mening

Under det givne
øjeblik hele den
levende sandhed
I lyset
gennemstrømmende alt,
bærende livets 
vilje



Juni 2005"),
("Hjertets slag",8589934592,"Modal overvejelse

Jeg kan så jeg
bør, men se om jeg
gider
Jeg vil ikke!

Må jeg det?
Tør jeg det?

Gider du lige!



August 2005"),
("Hjertets slag",33562624,"Målfast

Hækkene vokser i
lige linjer under
den åbne himmel
Hvide fugle
krydser de opmålte 
territorier, 
cirkler over det
klausulbehæftede på
fuldfjerede vinger
Glidende skygger på 
afmålte bede


Juli 2005"),
("Hjertets slag",2,"Naragtig

Den største nar er 
den der lever det
jeg dræber i 
mig selv



Maj 2005"),
("Hjertets slag",4294967312,"Natsværmere

Dueblå favner natten
Vejene mødes ved
den gamle hyld der
villigt åbner sine
drømme  mellem 
duftende fangarme
Fjerne kloder
kommer den
forpinte længsel nær 



Juli 2005"),
("Hjertets slag",0,"Navnløs

Selvom man ikke
ved hvad det er kan
man føle det
Selvom man ikke kender
ordene kan man mærke
smerten 
- alligevel er det som om et
navn udløser en
forståelse
Et ansigt at
forholde sig til



Maj 2005"),
("Hjertets slag",33554464,"Navnløs

Som strakte han armen ud mod
noget kommende
Som udstrakt i en stilhed uden
mulighed
Som fastfrosset i et råb
berøvet sin lyd

Kun træets skygge vover en
flimrende berøring
som et kærtegn uden
kontakt

Som havde intet været
Som fortiden var
løbet tom for indhold

Som var alt uden
betydning og dog var det
de samme hænder der  
engang rakte efter solen

Som om det var
bemærket et
sted

Som havde det
betydning



Juli 2005"),
("Hjertets slag",0,"Nok

Så er det
nok!
Nok er nok og
når nok har fået sit
er der alligevel ikke
mere tilbage


Juni 2005"),
("Hjertets slag",17179901952,"Nu ser vi

Ikke lys, ikke skygge
ikke glæde, ikke sorg
Ikke den brændende fakkels
vilje, ikke frostens
stumme vold
ikke livet inden døden  
ikke spring eller
modstand
ikke kærlighed
ikke had

Måske




Maj 2005"),
("Hjertets slag",2,"Nytænkning

Kan man ikke ændre
virkeligheden efter sine
ønsker
kan man komme ganske langt ved
at ændre sine ønsker efter
virkeligheden



Juli 2005"),
("Hjertets slag",618542432256,"Nærmere

Så nær som
vinden mod huden
nej, nærmere endnu
som blodet der når
hver enkelt celle,
som tankens dialog
med  universet
som længsels leg med
uendelighed
kom nærmere 
kom nærmere end
jeg



Juli 2005"),
("Hjertets slag",549856477186,"Nærmere til dig

Du kommer tættest på
dig selv -
i  en anden


Juni 2005"),
("Hjertets slag",0,"Om kristendom

Kristendom drejer sig ikke om at
gøre noget
Kristendom drejer sig om at
være den man allerede er
og glæde sig over det
man allerede er blevet givet



Juli 2005"),
("Hjertets slag",0,"Omkostning

Vil du skemalægge livet
udskiller det angst der
ætser alt


Juni 2005"),
("Hjertets slag",33562626,"Opdagelsesrejsende

Vi forcerer bjerge og
krydser kontinenter for at
komme tættere til
os selv



Juni 2005"),
("Hjertets slag",65794,"Opdragelse

Vi har så travl med at
retlede næste generation at vi 
ikke ser  at det er
vore egne fejltagelser vi 
straffer dem for



Juni 2005"),
("Hjertets slag",1081344,"Orange

Lettelsen er så
stor at mine ben slipper
jorden og tillader mig at
forsvinde i den
blå luft

Blinde høners
saltomortaler!

Vuggende i salighed
piller jeg min
appelsin


Juni 2005"),
("Hjertets slag",2129920,"Overflade

Under mine ord ligger
havet, sjældent i 
ro men dette nu spejler
silkeblankt og 
smutter blanke smil af
solen


Juli 2005"),
("Hjertets slag",135266308,"Overførsel

Birketræets nøgne
knopper mod
himlen 

Min dansende drage der flirter med de

ladte skyer og
alt  
som brister ud i

tændte blade og stormløb på
livet 
Gennem den spændte snor
modtager min krop den levende
kraft der engang skal sprænge mit
univers

Endnu engang og det 
gyldne lys forsikrer at det 
er alt


netop nu



Maj 2005


                  "),
("Hjertets slag",0,"Paradoks

Jeg vil ikke
finde mig i det
men det er det
eneste jeg vil
så jeg finder rum hvor
jeg ikke troede der var
plads og flytter om til
det bliver muligt
Er det bare det der
skal til


Maj 2005"),
("Hjertets slag",549890039808,"Plet

Livet
dirrende på sin
spids
endnu varmt af
kasterens hånd


Juli 2005"),
("Hjertets slag",77309444096,"Regn

En regndråbe
En kind
Regndråbers grådyb 
mod mængdens ansigt
En dråbe 
en kind eller 
er det en
tåre




Juni 2005
                  "),
("Hjertets slag",618592739328,"Relationer

Der må være et
jeg og et du
Jeg og jeg bliver
for stort eller for
småt og
du og du så
forfærdende ensomt


Juli 2005"),
("Hjertets slag",2,"Revision

Bliver dit livs revisor ved med at
vende tilbage til
gamle regnskaber skal du
omgående skifte ham ud med en
anden der er mere
fremadvendt



Maj 2005"),
("Hjertets slag",557184,"Rod

Fuglene trevler dagen op og
lader den hænge med
løse ender
Ellers faste aftaler
er havnet i filtrede tjavser og
de koncentrerede øjeblikke
faldet af
Måske kan jeg redde den 
røde tråd  hvis
jeg leder mellem de 
splittede lag



Juli 2005"),
("Hjertets slag",16777218,"Selvindsigt

Man kommer ikke så
langt ved at kigge i
spejlet
Kalenderen ved
antagelig bedre
besked


Juli 2005"),
("Hjertets slag",551903297536,"Signatur

Solskiven pudset og
på plads
Fuglesangen startet som
planlagt
På den ferskmalede himmel
har nogen skrevet en
autograf på 
kanten af
evigheden
- Det ses dog  næppe hvis
man ikke lige ved det



Juli 2005"),
("Hjertets slag",1048580,"Sku' vi?

Sku' vi så se at
blive voksne!
Ja sku' vi nu det?
Sku? vi ikke hellere forsøge at 
finde  tilbage til 
dengang vi var kloge og 
tænksomme uden forsøg på at 
leve op til idealer hr. og fru 
Nogen har fundet på,
springe rundt som gamle
unger og  stikke næsen i
ting der ikke kommer os ved
Tage imod dagen som den
er  fordi vi har erfaret at
den  alligevel aldrig ter sig som 
vi kræver
De små er måske de eneste der 
for alvor ved hvor 
farlig verden kan være og 
hvor fantastisk et sted når man
har de rigtige øjne og 
opfatter uden filter og 
som ikke er bange for at stå på
hovedet hvis det er det der skal til.
Så små at 
de  kun kender tillid og
bliver ved med at møde  ting der er
forunderlige nok til at de
glemmer sig selv
Hvad med at gå i graven som
hujende småkravl der er i gang med at
fylde sig med  glæde og langtfra har
fået nok endnu




Juni 2005"),
("Hjertets slag",4294967296,"Solglimt

Skarpe vinger klipper
dufte af havet  og lader dem 
drive på  lyde mellem 
skrig og fløjt 
Og fra et grænseland frigøres
en evig sommers 
latter mod
brunet hud,
den ubekymrede glæde og 
længslen i nattens
dufte mellem fiskernes
udspændte net



Maj 2005"),
("Hjertets slag",549755822082,"Spilleren

Den der gav kort
er magtesløs
Kun håndens ejer kan
forhandle med den
blinde vilje


Juni 2005"),
("Hjertets slag",167772672,"Spire

Døren åbner sig
for mørket og han
går ind som 
en del af
rummets tomhed
En kapsel lukket om
sig selv  
En hjemløs
erindring

Når gråden bliver til
morgendug
spirer lyset frem mod
dag



Juli 2005"),
("Hjertets slag",2048,"Strømmende

Folkestemningen løber som
vandet den  
lettest fremkommelige
vej gennem 
terrænet  på
jagt efter freden i 
det fælles hav



Maj 2005"),
("Hjertets slag",16785408,"Surfer

Når du læser dette
er jeg ikke mere som
den  der var i netop
denne tanke for
kun ordene lader sig
genbruge ud over den
sidste holdbarhedsdato og 
har da heller intet forsvar mod 
en tid der ikke er 
deres egen


Surfende på nuet
forsegler jeg en
tanke


Juli 2005
                  "),
("Hjertets slag",68719476736,"Sår

På vej væk en
drejning af
opmærksomheden,
nok til at
registrere flugt 

De sår man
tilføjer sig selv
har svært ved at
læges



Juni 2005"),
("Hjertets slag",549755813924,"Tro

Før var livet altid 
- uendeligt
nu ved jeg at
alt levende skal
dø 

at livet kendes på
døden

Men under stjernes
fjerne skrifttegn og i 
lyden af 
vindens hvisken 
føler jeg livets 
udødelighed -
at alt der har tro som 
et sennepskorn skal 
vokse gennem
døden til sig selv og
livet i lysets 
evighed


Juli 2005"),
("Hjertets slag",17179870208,"Type

Han var ikke af
den slags man
laver mandfolk af
Snarere juletræer -
af den type der 
selv har sørget for
pynten



Juni 2005"),
("Hjertets slag",8757747712,"Ubekræftede øjeblikke

glidende skyers bane mod
horisonten

ubevogtede tankers passage af den
rolige baggrund

vandrende skygger over
det åbne landskab

du hvisker det til
natten
lader det forsvinde
over  havet
du overlader det til
den åbne
jord 



Juli 2005"),
("Hjertets slag",0,"Ubetinget

Grib det i
flugten og
hold det varsomt,
flyv med hvis du
kan eller lad det
fylde dig hvor du
er men mist aldrig troen på
at det at leve er
en ubetinget gave der
tilhører alle med 
samme ret


Juli 2005
          "),
("Hjertets slag",135266304,"Udfordring

Kan man komme
for sent til livet eller
stå af i Roskilde -
måske kan man det 
selv om det  har en 
forunderlig evne til pludselig at 
springe frem fra en 
busk med frække øjne og 
spørge om man er 
til  noget -  og
skulle man få sådan en 
udfordring er det vel aldrig for 
sent at springe til.




Juni 2005"),
("Hjertets slag",0,"Udredning

Det behøver ikke være på
grund af brevsprækken
postbudet går
forbi


Juni 2005"),
("Hjertets slag",16777220,"Uhindret

Der er ikke så
mange spørgsmål mere
Det uafvendelige har
indtaget scenen og
egentlig er det
lettere på denne måde
Landskabet er bygget af
elementer der ikke lader sig
flytte  og  deres mellemrum
hvori mit liv flyder uhindret mens
jeg i det stille støtter de
kræfter der aldrig vil 
nå i mål mens 
jeg er til
Kun ømheden er blevet
større,
forståelsen og 
tilskyndelsen til en
accept jeg ikke 
kan skimte enden af



Juli 2005"),
("Hjertets slag",0,"Under himlen

Vi går gennem gaderne
til markerne 
under himlen og  da vi 
vender os ser vi
byen som en 
bræmme med 
insisterende tårne og
vi er ikke længere 
mellem de andre men
bliver til os selv og lader os
vokse ind i konturer vi
ikke kender rækkevidden af
i den uhindrede stilhed 
Dagen tæller ikke timer men
regner i lys og skygger og
først da rummet har
foldet os ud  lader vi os
vende tilbage til 
udgangspunktets slidte tryghed og 
giver det lys af 
vores lys mens vi
ifører os dragten  der 
tager imod os som
handskens forsikring til 
hånden






Maj 2005"),
("Hjertets slag",549755822084,"Vandringsmanden

Han havde vandret på
må og få,
hid og did som vinden
blæste
Da han mærkede enden var
nær vendte han sig og
så sine spor der gik i
en lige, målbevidst linie som
trukket langs en
lineal


Juli 2005
                  "),
("Hjertets slag",67272720,"Vinca

Din farve er hverken
himlen eller havet -
snarere erindringen  om
et par øjne jeg
kulsejlede  i
- engang



Maj 2005"),
("Hjertets slag",1056768,"Vips

I det øjeblik du
griber efter
meningen med
livet er den væk
Når du fylder 
hænderne  med 
hverdag er den der igen
som en fugl over
dit hoved


Juli 2005
                  "),
("Hjertets slag",16949264,"Ærenpris

Langt ude ligger
horisonten som en
postuleret streg mellem
himmel og hav men
lige her eksploderer
livet i blomstring
uden forbehold for
udlængsel eller
drømme om andet end at
fylde pladsen ud og
løfte livet frem af 
mulden 

Søgende eksistenser foran
flygtende horisonter,
et sted mellem himlen og
havet og farven er 
næsten den samme


Juni 2005"),
("Jordens liv",755981451268,"Adskillelsen

Smerten
Sandkornet
Hjertet
Mørkets gnistrende
himmel
Evigheden ses
ikke i dagens lys
Jeg længtes efter dig
længe før din fødsel og
for altid
Og det er smerten der
lægger lag på lag for at
danne den
strålende perle
ingen kan se fordi
jeg bærer den
dybt i mit 
hjerte




Juni 2006"),
("Jordens liv",77426860032,"Aktører

Bag landskabet ligger et
andet landskab
Bag dit smil et 
andet smil og  vi 
siger de ord der er
blevet sagt før og
som betyder det
samme men ikke er
det samme fordi vi
blevet fremmede for
os selv i et fremmed
landskab i en
fremmed tid
Vore egne ord er som
påstande der har 
revet sig løs og 
svæver rundt uden for 
rækkevidde og
vi vil at filmen skal
slutte men kender ikke
vejen til det 
tabte land og må 
fortsætte spillet som
slidte aktører der 
ikke længere kan 
skelne hvor  de selv 
begynder og hvor
rollen holder op



Marts 2006"),
("Jordens liv",117473794,"Bankerot

Man kan blive så
bange for at
miste at man ikke
længere kan elske -
og så har man
mistet alt


Maj 2006"),
("Jordens liv",558648788992,"Befrielse

Lyset fanger en
dråbe og lader sig
falde på den
tørre jord 

Hvad andet skulle
åbne mørket 

Så mørkt er 
mørket,
fanget i sig selv

Det, der har lukket sig
omkring sig selv kan kun 
åbnes udefra



Maj 2006"),
("Jordens liv",8640831488,"Brilleglas

Markerne kender kun
ét svar og
skovene er lukkede om
deres indre
disciplin
Gennem tilslebne
brilleglas gentager han
sit spørgsmål
En fugl på
lasede vinger
følger sin rute
uden at dele
sit overblik


Maj 2006"),
("Jordens liv",25836945920,"Colombusæg

Hun vil ikke
have ham om
dagen men
gerne om natten
for inderst inde er
hun to og
hende om dagen
har en anden smag end
hende om natten og
så er det svært at
slå sig til ro med
ham man vil have om
dagen men ikke om
natten og det er jo
derfor hun er
tvunget til at have dem
begge to



Juni 2006"),
("Jordens liv",549890074640,"De

De faldt på
stengrund med
god vilje 
De kæmpede forgæves
blandt tidsler og
lod sig æde af
sultne fugle
De så aldrig den
gode jords høst



Juni 2006"),
("Jordens liv",8727855616,"Den grå klit

Det skarpe lys
finder gennem lag
bølgeslaget har
åbnet
Skrigende fugle
fanger smerten og
forsvinder  over
havet  gennem  huller i 
det blå
Så lys er verden og
stien viser hvor andre
har fundet vej



Maj 2006"),
("Jordens liv",13036978180,"Drift

Den halve måne er
nok og  vinden netop så 
stærk at den 
løfter mine drømme over
mørket
Syrenen dufter i nat og 
syrener ved hvad de gør
Der findes intet 
fordi og dette
ender ikke med morgen
Selv månen forsøger at
slutte en kreds
Så kort en vej til
den form himlen har
tegnet længe



Maj 2006"),
("Jordens liv",1073745408,"Dømt

Man kan sagtens
være dommer hvis
man nægter at
tage stilling til
tiltaltes forklaring


Juni 2006"),
("Jordens liv",35684868,"Elegi

Solen en skive
messing
Havet et
nystrøget tæppe
uden folder
Stranden belagt med
skarpe splinter fra
et helt liv
Drivtømmer uden
sammenhæng


Maj 2006"),
("Jordens liv",67142656,"Engel

Jeg betragter nøje 
og tænker
Jamen engle,
de har da vinger



Juni 2006"),
("Jordens liv",77376553474,"Ensom

Hvem er mere alene end den
der sidder med den eneste ene og
gerne vil være sig selv



Juni 2006"),
("Jordens liv",134225920,"Fodfæste

Ingen  lever mere
intenst end den der
har vundet fodfæste på
nuet og surfer af sted på
tidens strøm


Juni 2006"),
("Jordens liv",68786585600,"Forgyldning

Hvis jeg forgylder
din 
og du forgylder
min
så sidder vi på to
forgyldte



Juni 2006"),
("Jordens liv",287762841600,"Fortrolighed


Ikke tårer

Dråber

Ikke kinder

Blade

Blide smæld under
træet mens jorden
deler sin smerte 
med natten


Marts 2006"),
("Jordens liv",1048704,"Forudsætning

Det er vel sådan med
de mange grå dage at
de udgør en slags
grobund for de
lysende blå, de
tindrende øjeblikke
der kun lader
glæde tilbage


April 2006"),
("Jordens liv",17230234112,"Fredløs

Hvordan skal hun vide
hvornår de kom
De var der vel
altid og har set det
hele
Hun forholder sig
roligt og
gør sig så flad hun
kan  for ikke at vække
det mishag der 
har korsfæstet hende
til angsten
Men øjne husker og
sender hende billeder af
fejltagelser der ikke 
måtte ske
Hvorfor lo hun så højt når
hun kun skulle smile



Juni 2006"),
("Jordens liv",8589934592,"Frås

Fire linjer for at
udtrykke dette
Tre havde måske
været nok


Maj 2006"),
("Jordens liv",558479966208,"Fuglesang

Den nye verden
vokser frem af den
gamle mens det 
altid nyskabte liv
bygger et fundament af
det brugte 
Vi ved hvad vi 
vil vide og 
lader som om vi
ikke forstår
fuglenes sang



Marts 2006"),
("Jordens liv",137476735236,"Gensyn

Bag sløret ligger
byen som den var
dengang og da det
først er væk kommer
smerten ved at være
en fremmed i
sit eget hjem,
en ukendt der går
forbi vinduer der
har glemt hvem de
er,
en overlevende der
rastløst søger tegn på
liv,
et nødråb til en verden
der har glemt sit eget
sprog og ikke kender til
det liv der endnu kan
anes som skygger
på de stumme mure


April 2006"),
("Jordens liv",206158545156,"Havn

Engang men ikke
mere
små hænder der
greb efter mine,
ankertove,
livliner, som jeg
klyngede mig til af 
længsel efter det 
bolværk der skulle
have været mig


Maj 2006


                  "),
("Jordens liv",558630969856,"Helhed

Jeg er angsten for
det uforudsete,
de flimrende billeder af
katastrofer,
det totale forlis der ikke
levner håbet en
chance
Jeg er overfladen der
dækker
havet og spejler
himlens drivende
skyer
Silkefint gengiver jeg
det evige blå og
lader legende hvaler
gennembryde mit
panser

Af knuste hjerter
samles lysets puslespil


April 2006"),
("Jordens liv",549755813888,"Helliget blive dit navn 

Så længe har jeg
skrevet mit navn på
dit at jeg ikke mere
kan viske det rent
Hvordan skal jeg
kende mig selv hvis
jeg ikke  kan
glemme mig selv 
i dig


Maj 2006
                  "),
("Jordens liv",9697273858,"Hjertemure

De mure vi
bygger af
sten og stål er
for intet at regne
mod dem vi 
bygger i
hjertet


Maj 2006"),
("Jordens liv",50364416,"Hjørne

Dagen er et stort
rum med
mange hjørner 
Jeg fandt det der
ligger længst væk
Jeg kommer nok  igen
i morgen



Juni 2006"),
("Jordens liv",561970718848,"Håbet, troen, kærligheden

Hæv blikket 
til den lyse dag
Se, natten lader kun en
luftig drøm tilbage 
og klodens sårede samfund 
med levende bomber og
hjerter af sten
har brug for det håb der
fødes igen 
hver eneste morgen 
som tændsats for troen på 
en udelte verden, 
levende af den fælles
kærlighed



Juni 2006
                  "),
("Jordens liv",2260992,"Ikke kun

Så rød er kun
den røde
så blå kun den
blå og 
jeg kan ikke
nøjes med mindre
for jeg har set
havet vidne foran
himlens ulmende
hjerte



April 2006"),
("Jordens liv",117473792,"Jo

Det vil du som
ikke hører os  til
Og du må
gøre det selvom det
flænser vore
hjerter for også du
skal jo blive hel



Juni 2006"),
("Jordens liv",558481014788,"Jordens liv

Som en uimodståelig bølge
skyller livet ind over
det døde ler og lader det
leve i brændingens brus
inden det trækker sig
tilbage til sit ophav
mættet med 
jordens høst
Så meget lever i lyset 
Og med en smil må
jeg indrømme at også
med mig får
alderen sin vilje
Dagene er som bladene på
træet
nogle i sol og
andre i skygge
Et efter et giver de
slip og lader sig blæse i
vinden for at blive ét med
den jord der gav dem stof
og hele den levende
erfaring står skrevet i
lysets leg med den
dugvåde morgen 
der
får stemme af
livet i den form det 
nu har



April 2006"),
("Jordens liv",1124114432,"Kamp

Jeg ved hvad der
gemmer sig
bag månen
men er tavs som
den lyttende nat
Visse ting skal man
holde for sig selv



Juni 2006"),
("Jordens liv",559687967232,"Komme dit rige

Hvem kender vel
mørket bedre end
den der bærer det
i sit indre
Hvem kender dets
styrke bedre end
den der benytter sig af
dets kraft
Hvem kender vel
længslen bedre end
den der selv gav
mørket magt



Maj 2006"),
("Jordens liv",8875188736,"Kontrast

Det er ikke kun
drømmene, det er
også den onde vilje,
driften mod destruktion
jeg bærer  i den 
hånd jeg
skjuler bag ryggen
Det er ikke kun
den frie tanke
det er også den
kolde strategi jeg
har gemt et sted
bag ordene og det
venlige smil jeg
kan påtvinge mit
ansigt
Det er ikke kun
ondskaben, det er
også mig


Maj 2006"),
("Jordens liv",1026,"Kundskab

De fleste kan
alfabetet men
hvor mange kan
læse


Maj 2006"),
("Jordens liv",2,"Kvalificering

Én egen indsats er vel
aldrig et dårligt
argument


Marts 2006

                  "),
("Jordens liv",26340270592,"Løber

Han løber frem mod
solen og
han er ikke den
sidste og
vejen under ham er ikke
vejen men
føddernes gentagne
kontakt med distancen 
Og han løber
mod lyset der 
omsider har
fundet ham og følger
hans bevægelser han
ikke føler men tanken
der har taget ham og
sender ham gennem de
scenerier man ikke kan
lægge bag sig men
må løbe igennem
igen og igen og det
er ikke erindring men
smerte og smerten
trækker ham frem mod
den blændende skive
der vokser og 
vokser til den
bliver sort



Juni 2006"),
("Jordens liv",551300401154,"Middel

Kærlighed antændes
med kærlighed,
aldrig sprængsatser


Juni 2006
                  "),
("Jordens liv",6,"Minder

Det erindrede er 
hændelser
rykket fri af deres
baggrund og
gennemlyst af en
fremmed tid



April 2006"),
("Jordens liv",549891080192,"Mirakuløst

Man tænker: Herregud,
er det alt? -
indtil det går op for én
hvor utroligt det er:
Der er jo kun den
nøgne jord - og det
der føjer livet til


Maj 2006"),
("Jordens liv",25820176384,"Nul

Hvis ingen taler til mig, 
er jeg så uden betydning
Hvis ingen lytter til mig,
er jeg så uden værdi
Hvis ingen ser mig,
er jeg så  fortabt
Hvis ingen tænker på mig,
er jeg så overhovedet til




Juni 2006"),
("Jordens liv",558481014784,"Overskud

Som en tanke der
farer gennem dagen
uden at finde et
sted at folde sig ud
Som en krusning af en
vandflade der ikke
kender sin egen
dybde
Som en kærlighed der
ikke ved af sin
egen styrke og
lader sig forvandle til
smerte
Som den blide tro der
skænker sin tilgivelse før
den er fordret



Marts 2006"),
("Jordens liv",137438986752,"Pastorale

Som en blidt
mumlende strøm er
mit liv men et
sted i skyggen står 
en sjat sorg i et 
glas der venter på at
blive tømt -
gudved hvem der
skænkede op



Maj 2006"),
("Jordens liv",9663710720,"Pianist

Sit livs smertepunkter
har han arrangeret i
et klaviatur han
behersker virtuost
På gode dage flyder
smerten frit i timer
Han er aldrig kræsen i
sit valg af publikum der
må lytte til hans mesterlige
improvisationer over 
lidelsens tema og 
skønt  han gør sig
mindre og mindre
vokser han i stolen





Maj 2006"),
("Jordens liv",69793219072,"Plukkeren

Kun tøvende
plukkede Jeg
livet og så 
først med tiden at 
jeg tog det 
fra dig

Jeg så dig ikke

Hvad skulle du da også  
lige der?



Juni 2006"),
("Jordens liv",275012657152,"Poplerne

Poplerne tegner
lyde i støvet,
en evig bevægelse der
aldrig lader sig
standse
En indforstået raslen der
bliver til brøl af
nødvendighed

En blåsprængt frostdag
ringer katedralens klokker
med sølvlyd i den
tidlige morgen

Fugleflokke bryder
de  lange streger på
himlen og lader
sig falde til ro
det øjeblik

Det er vigtigt



Juni 2006"),
("Jordens liv",1073744384,"Refleksion

Hvor mange finker
kan der ryge af
panden før panden
er tom?

Man skal omgås sine
finker med behændighed



Juni 2006"),
("Jordens liv",558481055744,"Rejsende

Streger på himlen som
vognspor over
ukendt land
Håbet flænser den
kendte orden og
vender aldrig tilbage i
eget spor
Over de øde vidder
lyser troens lue og
tænder bål i
hjerter der
banker sig vej 
Også de der aldrig kommer
frem finder et hjem



Juni 2006"),
("Jordens liv",8724193284,"Rutine

Det er sidste gang og
vi ved det ikke
Vi gør det bare for
sidste gang som om
det var en selvfølgelig
ting at gøre det vi
aldrig kommer til at
gøre igen men
bag den grå rutine
vokser angsten for livets
bevægelse mod
forvandling, den evige
strømning der bærer
livet selv

April 2006
                  "),
("Jordens liv",8606744576,"Samklang

En  streg uden ende,
uden begyndelse
En sammenhæng
vibrerende som en 
streng, spændt til 
den tone der
har farvesat mit jeg og 
skabt mit behov for 
harmoni med netop 
denne svingning
Min dybeste længsel
klinger med og
forsøger at
sætte ord på det
ordløse der ikke
lader sig indkapsle i 
det der er 
fattigere end 
det selv


Juni 2006"),
("Jordens liv",68719510528,"Selvrealisering

Så flyv da hvis
det er det du vil og
gangen på jorden
ikke er dig nok


Juni 2006"),
("Jordens liv",33280,"Sjælebrand

Når sjæle brænder
er alt i bevægelse
og ingen kan
forudsige om det
ender med
forkullede stumper
eller glødende kul



Juni 2006"),
("Jordens liv",558497308672,"Skaden 

Det sorte blik  
lodder dybden af
min tro
Halen vipper sin
nøgne indsigt
Så blå er himlen



Marts 2006"),
("Jordens liv",0,"Skam

Solen har fundet
hullet i hækken og
sender et
gloende øje ind
Jeg ser væk som
var der nogen at
skamme sig for



Juni 2006"),
("Jordens liv",833375698944,"Ske din vilje

Grenene over mig i
stadigt finere
forgreninger som
blodårer der finder
vej fra det pumpende
hjerte til cellernes
utrættelige virksomhed
Bladene der filtrerer
lyset og toner dagen
med en sang jeg 
giver uhindret adgang
til rum som end ikke jeg
selv har betrådt
Kronen ovenover og
roden så intimt lejret i
jorden at intet kan
trække en grænse
Det frie åndedrag der
drikker universets kraft
og giver alt for intet og
intet for alt
Så lys er dagen at
dens vilje sker og
når natten kommer vil
himlen lyse af stjerner



Maj 2006"),
("Jordens liv",33280,"Skeløjet

Vi rækker hinanden
hånden
skønt kroppene ved
bedre

Øjnene er stået helt af


Maj 2006"),
("Jordens liv",1056770,"Skridt

Det ene skridt fører til
det næste og man går vel
altid frem selv når man
går tilbage


April 2006"),
("Jordens liv",32768,"Skål

På et øjeblik løber
mine hænder fulde af
lykke, en uventet
skålfuld lyksalighed der
ikke lader sig holde men
dryppe ud mellem
mine fingre og
siver væk i
en pyt

Hvorfor drak jeg
dog ikke?


April 2006"),
("Jordens liv",558481571840,"Spiraler

Vinger over byen i
glidende spiraler,
hvide som skum på
havet, befriet for
tyngden  som tanken der
slider sig løs og
glider på frihedens
levende kraft og
bliver til en bøn
om alt eller intet,
ordløst stigende 
mod øjets og
sindets grænse i
håb om forening med
livet selv


Maj 2006"),
("Jordens liv",8740970528,"Sporskifte

Ansigterne passerer forbi på
gaden, svævende som 
balloner der
venter på en chance til
at rive sig løs og
forsvinde op over
de vandrende kolonner
Hvor grå kan en
sjæl blive inden den
duver op i det
blå og springer ud i
sig selv uden trækket fra
snoren der holder igen og
holder fast mellem
alle de andre der har
glemt hvem de er eller
aldrig har anet
muligheden af flugt
Øjnene undviger bestandigt
hinanden og skønt
mundene kan tale
røber de intet og
skifter ustandseligt spor


Maj 2006"),
("Jordens liv",69810006018,"Stadier

Vi er som børn der
endnu ikke har
brændt sig nok


Maj 2006"),
("Jordens liv",12953092096,"Stilhed

Denne lyd af
intet
Stilheden behøver et
bølgeslag,
vindens leg med 
græsset og 
som et stadigt ekko af
livets velsignelse
lyden af dit hjerte



Maj 2006"),
("Jordens liv",4295000576,"Sult

Himlens skyer er
hvide i nat og
driver for den
lette vind
Så let at lade sig
drive i mørket
uden afsender eller
adresse
som en afrevet
lap papir,
sulten efter 
bogstaver



Juni 2006"),
("Jordens liv",1342180354,"Talent

Kunne man erobre
kærligheden og
tolerancen med
bomber ville vi
for længst have 
indtaget paradiset på 
Jorden 


Maj 2006"),
("Jordens liv",10738466944,"Tillid

På en sommerfugls vinger
letter min dag og
lader sig gribe af
vinden der kun vil
sig selv
Himlens luende tulipan 
ser til
Disse vinger kender  
deres styrke der er
klippet af lidt
himmelblå



Maj 2006"),
("Jordens liv",32804,"Timeglans

Sandet løber mellem
mine fingre uden 
ophold
Kan man stole på
der er nok?


Maj 2006
                  "),
("Jordens liv",1090555392,"Total

Hør, det er som et
nødråb i mørket
Se, jeg har fjernet hele
mit forsvar og
skjuler mig kun
bag min skam


Marts 2006"),
("Jordens liv",16777220,"Tre

Erobre
Besidde
Miste


Juni 2006"),
("Jordens liv",68719510528,"Tro

Jeg tror at du
tror og det
skal du ikke for
hvis du tror
hvad jeg tror
tager du fejl
Og nu tror du jo nok at
jeg tror men 
det gør jeg ikke



Maj 2006"),
("Jordens liv",33587712,"Tydelig

Stjernerne vil ikke som
jeg og lader sig
ikke tyde
De klare mønstre
taler deres eget sprog
over mit hoved mens
jeg bærer min
byrde hjem hvor 
ingen venter og jeg kan 
tænde husets lamper
som jeg vil


Maj 2006

                  "),
("Jordens liv",77309411392,"Tærskel

Døre der åbner sig
Døre der lukker
Anonyme døre der
venter på handling
inden de afslører
deres hensigt
Skæve døre der
ikke længere
lukker tæt og
døre der stiller
betingelser på
ordknappe opslag
Hængsler der
bærer og muliggør
bevægelse
Indenfor udenfor og
altid denne omstændighed


Marts 2006"),
("Jordens liv",8256,"Tærskler

Der er det
store farvel -
og de små
De mange tærskler vi
må passere for at
nå frem
De utallige døre der
skal lukkes før andre
kan låses op


Maj 2006"),
("Jordens liv",137438994948,"Uforløst

Som en perle i
en musling
den gråd der
aldrig blev
forløst


Juni 2006"),
("Jordens liv",558346797060,"Undervejs

Hvert skridt forkorter
vejen gennem
tågen og lader den
lukke sig om det vi 
gik forbi
Vi kommer klædt i
vores erindring og 
det er vel et 
spinkelt håb vi 
bærer med os som en
varsomt forvaret plante vi 
håber at kunne give et
hjem når vi 
engang når frem


Marts 2006"),
("Jordens liv",68719509504,"Usigeligt

Det fylder 
mellemrummene
Det ligger frit på
bordet men ingen 
samler det op i ord 
Kun øjnene kan ikke
holde afstand og kører
budskabet som en
lysavis der ikke kan
stoppes  fordi ingen kan
finde kontakten


Marts 2006"),
("Jordens liv",8589934592,"Vinger

Et fjeldmassiv
min hånd
hele jordens
styrke
en sommerfugl
min hånd 
et øjeblik  der ikke 
kun er mit og
denne skælven
under hele
klodens vægt


April 2006"),
("Jordens liv",16818176,"Vingeslag

Hvad er smukkere end
lyden af sommerfuglenes
vingeslag
- mere knusende end
vægten af det liv man
aldrig fik levet


Marts 2006"),
("Jordens liv",1310720,"Vinterdage


Revner af frost i
det stivnede panser
Den hårde jord
modtager ingen
aftryk og fastholder
gamle spor
En indkapslet dråbe
under en fuldendt knop


Marts 2006

 
                  "),
("Jordens liv",8875180032,"Voksen

Nu lægger mørket sig
om huset og gør mig
lillebitte og over
taget løfter himlen sig på
tæer med alle sine
stjerner
Der er så langt til
universets hjørner at
end ikke tankerne har
været der så ingen
ved hvad man kan
finde, måske en dynge
himmelsk rod selv engle 
går forbi 
Og  når mørket
slipper grebet ved det
første tegn på dag
kan jeg vokse ud
som voksen  og 
strækker mig
tilbage i 
mig selv
så langt der nu 
er plads


Juni 2006"),
("Jordens liv",275013214208,"Yggdrasil

Hjorten beskærer
livets træ med
hastige bid under
månens blinde øje 
Brudte håbs fundamenter
bærer tunge mure


Juni 2006"),
("Jordens liv",8589934592,"Økologi

Skovstier
Træer der 
kradser lidt i
himlen
Hunde med
mennesker i
snor og én der
forsøger at
løbe fra sin
alder
Et par der
har opdaget lykken
inden i hinanden og
forsøger at
komme ind
I ly af kække
graner lader jeg
mit vand og
hører jorden takke


April 2006"),
("Konserves",68719476736,"Øjne

Øjne der beslutsomt har
bidt sig fast i
bogstavkombinationer

eller flydende landskaber

Øjne der gemmer sig bag
sænkede vindebroer og
gør slangehuggende fremstød 
uden konfrontation

Søgende øjne der stiller
ublufærdige spørgsmål
og vil

Øjne der intet ser
udenfor kroppen de
betjener

Øjne der intet vil men
ser 


          "),
("Konserves",0,"Øjne

Livet står
udenfor og vil 
ikke ind 
Får jeg øjenkontakt og
lukker døren op har 
det gemt sig bag 
havens buske men
om et øjeblik vil det
stå der igen med
sit spørgende
blik
          "),
("Konserves",0,"2003"),
("Konserves",1310720,"Alarm


Tomme felter
kravler op
over bakken
og overtages af
himlen der har 
magten i dag
Den første
lærke råber
alarm til det
ventende forårs
glimt i øjne som 
slår vipper ned
for solens hede
blik 


                  "),
("Konserves",1073743872,"Ara

Et sted dukker en
ara ud af
bladdækket og 
sender sin
farves skrig i
omløb

Årtusinders ekko hamrer
mindetavler af 
epoker der troede lyset 
forseglet i evighed

Et skib undervejs mod
sit bjerg kaster anker mens 
dagen lister sig ind på 
den potente forandring der 
aldrig har spurgt om lov


22-12-03


          "),
("Konserves",0,"At 

Flyve som
fugle over
svangre
landkort
fisk gennem
sunkne byers
gader
flyve som
tanker uden
retning
som et blik
mellem to der
passerer hinanden
tomme for
ord

          "),
("Konserves",32768,"Båd


En sær
sørgmodighed
sejler ind i
mit sind
og lægger 
til
Kasser
losses på
kajen

Der kommer
en dag 

En villig 
udvej
sluger mine
skridt

Endnu er det ikke
kassernes 
tid


          "),
("Konserves",68753063936,"Båndet

Jeg kan høre dit
hjertes banken selvom
jeg ikke kender dig
Dine øjne fortæller 
resten som jeg heller
ikke vil vide af og 
uden at tøve kapper 
jeg båndet mellem 
os og går 
          "),
("Konserves",8388608,"Bag døren

Der er stole og borde
Bøger og billeder
Tæpper
Småting der ikke rigtig ved
hvad de skal og et 
fjernsynsapparat der længes efter
en friaften fra overfyldte 
askebægre og kaffesjatter i 
skårede kopper

Der er hvad der skal til
          "),
("Konserves",0,"Bag glas

Et øjeblik og et
ansigt 

et falmet fotografi og
et forseglet spørgsmål

en erfaring og et
forsinket svar

                  "),
("Konserves",32768,"Balance

Drømmeskyers drift
gennem dagens fokus 
splinter viljens
balanceakt ad
pligtens knivsæg
Parallelle universer
som duvende
balloner holdt i
snor af en
styrende hånd


          "),
("Konserves",566935683340,"Barnet

Han har båret
barnet gennem 
tiden
Den unge mands
løfter til en
stum vilje der
ikke ville som han
Nætters kamp mod
mørket og blomstringen der
gik i stå uden det lys
der et øjeblik fylder hans 
øjne og får barnet til at
le fra de 
visne læber


                  "),
("Konserves",17196651012,"Bedrag

Løst af tiden
driver du omkring
i kroppen der
pågående udstiller 
funktioner og
råbende afslører 
livets bedrag

Gennem ruder  
lever vi med og
genkender sjældne 
solglimt over dit
ansigt




                  "),
("Konserves",0,"Besat

Legende  undslipper tonerne
hændernes jagt og  
besætter følelser der 
ikke finder værn for
rytmens greb om 
kroppen
Over dagens kantede tabel
blomstrer svingende 
vibrationer i 
ustoflig eksplosion der 
sætter afviste længsler i 
brand
Skjulte perspektiver
åbner uanede indsigters
mulighed bag
klanges vibrerende
mure

Stilheden står
forude


          "),
("Konserves",695919968292,"Bladenes jord

Væksten vil det
Livet vil det
Den nødvendige
fasedrift vil
rene linier så
intet vraggods
skyller uforandret ind på
morgenens kyst
Døden vil det som 
en gave til 
den nye jord
                  "),
("Konserves",138646947856,"Blomstrende enge


Himlen folder sig om
stumme  bønner
Himlen -
hvor ellers

At spilde liv
Satan
Helvedes slikkende
flammer mod
skyernes apati

Lærkesang over
blomstrende enge 
ingen ser

Vuggende valmuers
flammede øjne i
mørket over en 
lille hvid
æske

Sammen ser vi
livets blomstring



                  "),
("Konserves",146070863908,"Boet

Stilheden holder
tungt om lugten af
liv der hager sig 
fast i pletter og 
skjolder på
nedslidte fladers
erfarenhed
En uredt sengs 
forstenede aftryk  af 
en der aldrig 
lagde sig blot 
udstillet for dagen 
parat på måtten med 
opsatte gøremål uden  
tid til blege minders bid i
ting der ikke har mere 
at gøre her


                  "),
("Konserves",35653632,"Broen

Lysfyldt græder
sommernatten over
havets månesvangre
spor gennem mørket
der ikke ved af 
anden udvej

Tomme armes
afmagt rækker ikke
over



          "),
("Konserves",0,"Buksbomskygger

Ad skyggefuldt passager
når I frem bag stilhedens forhæng
og står blot

som magtfulde vogtere af
liv der leves af levende

spiller I jeres roller men
undviger

tankens lys der alene belyser
en tid I ikke kender
          "),
("Konserves",2112,"Døre

dør efter dør kræver
sit spørgsmål
inden den lukker op

knirkende gulve
leder frem til

lukkede udveje der
bærer dobbelte
masker

mod den
søgendes
blik
                  "),
("Konserves",134217744,"Dammen


Nøkkeroser
svævende på
stilke under
drivende skygger
Vandfladers
sølvbelagte
hinder over
konturer af
levede livs
dynd i lag
der løfter
tegn fra 
mørket
ind i lyset
                  "),
("Konserves",1081344,"Danser

Danser over
husenes tage danser på
kanten af nattens
blå mens duggen
velsigner  med
bløde læber og
jorden har lært at
forstå

          "),
("Konserves",68720623880,"De små


Uden at spørge
om lov
sprang de ind 
i mit hjerte og 
byggede rede
af tillid og tro
midt i en
blomstrende
have skudt
frem af 
mit sinds 
frosne håb 
om kærligheds
magt til at
skabe påny

En livets
fornyende
pagt
                  "),
("Konserves",550024513536,"December

Luende  gennem
tågen i
horisonten løftes
hostien over tvivlen der 
ikke opgiver før den 
sidste mulighed for 
kamp er forsøgt

Evighed har tid til
at vente 

23-12-03
                  "),
("Konserves",214865840128,"Det handler jo om
kærlighed

Mænd der
begærer mænd
Mænd der elsker
mænd
Drenge der løber ud af 
foråret for at møde 
sommeren forude uden 
viden om efterårets
eftertænksomme trækken sig
tilbage til vinterens kulde
Ukrudt mellem fortovsfliser
Skabede græske køtere
der fornægtes med sten og 
velkomment studiemateriale for 
en ung forsker der 
bøjer hovedet over 
luppen døv for
generationers salmesang
En ældre mand der
smiler forsigtigt og
vover at stryge en 
fremstrakt hånd



                  "),
("Konserves",0,"Drømmende drømme

Drømme drømmer baglæns drømme
drømmer drømme jeg ikke
vil stå skulder med
drømmer baglæns gennem
drømme der ikke ville 
som jeg
længselsfulde drømme
drømmer drømme der 
ikke tør møde min 
dagligdags krav om drømte
idealer alle tør nå


          "),
("Konserves",85916155904,"Endelig

Jeg har sat filter  i
min telefon
der spærrer for 
opkald fra dem der vil 
skrive min dagsorden
Min brevsprække er
programmeret til at
spærre for 
henvendelser fra
folk der vil 
forbruge mit liv
Og allerede havelågen
nægter adgang for enhver
der forsøger at gå ind
uden vilje til at dele
min virkelighed et
øjeblik
Endelig er jeg
gammel nok til
at plukke livet 
selv med egne
hænder
          "),
("Konserves",137438953476,"Et livsforløb

tonet af overlappende
faser og skarpe brud
den bevidnede fødsels
fodspor til
den attesterede død
himlen er mørk derude
arret af et stjerneskud

          "),
("Konserves",558347878912,"Et øjeblik

Giv mig en
tanke at
fortøje ved
til jeg igen
vender hjem
til mig selv
                  "),
("Konserves",64,"Etager

Trapper der
haster opad mod
kopierede platformes
afrettede døre

Afrettede døre
foran  lovbundne
territoriers bortvendte
rygge
 
Bortvendt rygge der
lytter efter trin på
vej i skaktens
modløse ekko 

Nogen der finder
bestemmelsessteder
bag døre



          "),
("Konserves",34360786944,"For enden af
regnbuen

Uden tanke på
andet end glæden
vil jeg gå et
stykke vej uden
støtte af angsten
og ikke gemme
mine øjnes sol for
dig der kommer mig i 
møde
Jeg vil lade fugle
bygge rede i mit
hår 
synge med på 
vindens sange
og gå over regnbuen der
begynder der hvor vejen
ender
Jeg vil gå og ikke se 
tilbage mod de egne der 
lukker sig bag lyden af 
mine skridt


                  "),
("Konserves",135299072,"Forår

Efterårsløvet danser som
det skal i alleen før det 
må overgive sig til 
vinterjordens krav
Jeg knæler ned og
binder et skobånd der
ikke er mit mens en
engel purrer mit
hår og det er 
forår herfra og så 
langt blæsten vil
          "),
("Konserves",566935683072,"Forening

Du står bag
min tanke, 
toner mine
øjnes lys og
leger i min 
skygge  
Du smutter mellem 
mine ord og vejer
mine håb mod
skæbnetunge valg
Bag brændende
ørkener venter du 
dagen der 
samler mit
mod til at 
se hvem 
du er



          "),
("Konserves",0,"Fragment 

Jeg rækker dig 
ikke en gåde
Jeg rækker dig
mine ord der
bærer aftryk af
det jeg har 
mod til at miste
          "),
("Konserves",134217728,"Fragmenter

Sommerfuglevingers
lydløse jagt på fremdrift
Fragmenter af Edens Have på 
den anden side
uddrivelsens realitet
Sporløse viljers træk over
kontinenter i have der
skjuler glemte skatte 
Liv der ikke vil stå til
regnskab men undviger
mellem tabellernes
gardiner og slår sig til
ro som en lukket bog

Mørket gløder
intetheden øje mod øje
og lægger nye spor øde
for mening

          "),
("Konserves",618560258048,"Frikendt

Et øjeblik er
afstanden mellem os
smertefri og tiden
mellem to 
vejrtrækninger så
lang at du når at
række mig din
hånd og lægge din
tro i min

Og der er
tid endnu

29-12-2003
                  "),
("Konserves",549890031632,"Frø

Svævende
frø sejler 
gennem universet i
den tro at de
selv sætter kursen
mod en uforudset
landing der
skal forløse
høstens gave

                  "),
("Konserves",0,"Gemt


Bag lågerne
hænger lyse sommerdage
på dinglende bøjler
side om side med
indespærrede drømme
der aldrig fik lov
I glemte æsker
hviler bedøvede
tanker der venter
på deres tid under
låg
I lukkede skuffer
sover muligheder
rævesøvn i 
silkepapir
parate til
flugt
Bag lågerne lyder 
larmen af tiden 
der spiller
pingpong med
erindringer og
planer mens 
spirende håb 
ser til

          "),
("Konserves",1241645060,"Grå is

De nøgne vande
frosset til i
blinde lag der
ikke vil give sig
Livet forvist til
et dækket indre
afslører intet om
følsomheden der
tæt over bunden
forsøger at
holde ud med
længslen 

En brandmærket
gren mod den
grå is


22. september 2003 




                  "),
("Konserves",1081344,"Grænseløs

Jeg putter mig i
digets læ og
opgiver alle 
grænser

jeg bliver en
sang fra
jordens mund

et blomsterbed ved
digets fod

en duft der
blander sig med
vindens sang og
blæser væk i tiden

Lysende streger opgiver
himlens mørke



          "),
("Konserves",33587200,"Grønt lys

Lysets grønne hensigt 
folder sig om 
morgenduggen og 
løfter et ansigt

Nogen har glemt at
hente stilheden ind 

Søjlebårne rum står
op af skygger og
lader sig fylde med
spørgsmål der ikke
har turdet søge
deres svar


                  "),
("Konserves",2147491844,"Håndarbejde

Hver morgen
smider nok et
garnnøgle for mine
fødder for at jeg
inden aften kan
nå at strikke en
dags pinde på
den skjorte der
åbenbart stadig er
for kort
                  "),
("Konserves",17858560,"Høst

I solskyggen under de
gamle ege strør du 
dine tanker og lader dem 
gro som de kan
Kaster dine drømme
i vinden over den 
glemte voldgrav 
Fisker dine øjne fra
kanten af 
en  bro

I hegnet mod 
marken plukker du 
dig selv med 
nyfødte hænder


          "),
("Konserves",2050,"Hegnspligt

Sæt hegn om
de tanker der ikke
bør dø
før de har
blomstret og
spredt deres
frø
                  "),
("Konserves",8390656,"Huse


Husene
de selv har
bygget
Væggenes
ly for
ukendt liv
Spejle fanger
omfang
og rejser
monumenter
i ødet
Huse
står fast
i blafrende
vilje

          "),
("Konserves",146037278720,"Hvide gardiner

Støjen fra gaden
fylder havens ro til randen
løber over og vælter sig 
ind i huset der
vender ryggen til og 
holder på facaden
Lyden af stilhed længes
mellem folderne i hvide
gardiner der blev hængt op
til en anden tid  der
gemte svale ord  på
blanke reoler med
ryggen til vægge der
ikke vil forstå


20. september 2003
                  "),
("Konserves",134217732,"Indsigt

Du skal ikke forstå men
bøje dig og samle den
afgrøde der kun kan
høstes af dig


29-12-2003


          "),
("Konserves",68753063936,"Inkarneret

Gadens lurende 
farlighed
En ladning skønhed der 
faldt  efter en 
moden klase held 
Skrå strimler venlighed rullet 
ud  mellem høflige 
tilnærmelser og  en 
massiv blok 
kærlighed på 
tværs

Med pansret ansigt
slap han hjem 
knappet op i 
figursyet fjendtlighed


                  "),
("Konserves",68719511568,"Interviewet

Ordene fløj mod ham som
dartpile afsendt af en
sejrsikker mester der
uden tøven plantede
tyreøjet til med en
sværm glødende
tulipaner






                  "),
("Konserves",550846334976,"Jagten

Så snart jeg
hørte ondskabens stemme
sprang jeg op til
en livslang jagt
til jeg mødte
mit ekko og
slog blikket ned i
tavs accept


30-12-2003
          "),
("Konserves",1090521088,"Jeg

Jeg er

Jeg vil
Du skal

Jeg har
                  "),
("Konserves",65536,"Kæden

Der er dem der 
var her og efterlod sig
mulighed
Der er dem der 
kommer og 
griber efter 
livet hvor det er
Og der er 
kædens svage led

          "),
("Konserves",2048,"Kaos

Æbleblomster dryssende
gennem natten Titannic 
sank
Polarræve afsøgende deres
enemærker
Skorstenes dynamiske røgsøjler
udenfor værelset hvor en
fødsel afsluttes med 
skriget der 
sætter kaos på plads
          "),
("Konserves",0,"Konserves

Der er de
ord der skal
være
Det nødvendige
antal sætninger 
til at fylde de 
sammenhæftede
sider under bindet
Opmarcherede i
række under række 
konserverede  tanker
til fordeling
Balsamerede drømme
der fra deres sarkofager
udsender dufte af 
genkendelighed
Flygtige skygger mod
mausoleets vægge



 





          "),
("Konserves",4,"Løbesod

Hen under aften bliver
markiserne rullet
tilbage så lyset får
tag i de gamle
portrætter der
vågner op til dåd før
nattemørket tager
dem tilbage i
deres tid
Gulnede breve i
glemte æsker sender
følere ud i parfumerede
dufte der kradser på 
gammelt tegl og
blander sig med
lugte der ikke har sans
for raffinementer som
den gamle guldregn der
står fast på sit sted og 
skumler vinduet der 
forsøger at få det 
sidste glimt af dag



          "),
("Konserves",8421376,"Latent

Et hus rugende på
en smerte der ikke
vil klækkes 
Rum der ikke kan
falde til ro om 
deres bestemmelse
Ufuldbårne ønsker  vokset
vanføre  i det skæve 
lys uden potens til 
at sprænge deres 
hylster
En stilhed der ikke 
klarer at vente



          "),
("Konserves",32768,"Legekammerater

Glæden bør
lege med
sorgen uden
opsyn for 
sammen kan de
få ørkener til at
springe i blomst
          "),
("Konserves",0,"Lyst

Det er vel ligegyldigt hvad 
vinden mener om min 
lyst men uden tvivl er
den imod
Templets flag står
ret mens det slår 
smæld over skaren foran 
Moloks  gab 
Den hede ånde triller
toner ud og flirter med et
smil i karrusellens
mavedans
Sildestimers skrå kurs
mod lyset høster et
lyn af den sønderrevne 
himmel og lader mørket og
vinden stå 



          "),
("Konserves",10871638016,"Morgenland

I  dugvådt morgenland
står evigheden 

parat i lugte af  
liv der ryster gramsende
fingre af sig 
og spiller med 

på arvede
opsætningers fortolkning af 
oprindelighed

Blødt folder natur sig i 
menneskers tomrum

de må forsvare mod
lysets blide magt  

I monotone korridorer
søger den blinde vilje
vækst 

mod duggens 
velsignende hinde 

fugleskrig skårer himlen
over 
på  udspændte
vinger


                  "),
("Konserves",8590491648,"Mure

Nattergalesangen
over den glemte mose
rejser nattergalemure og
vælter mine

Et skært øjeblik
træder jeg ud af
mit tilkæmpede
fængsel og ser

                  "),
("Konserves",6710886912,"Nat

Natten ligger rullet
sammen på puden og
og venter på morgenen der 
holder sig væk til de undslupne
tanker har fået nok og er
vendt tilbage til deres æsker 
Mørket står tæt og lytter

                  "),
("Konserves",567069900804,"Nok

Årene
fordampede under
blodets hede og
slap næsten
op før du med
ærefrygt førte den
sidste dråbe til din
visnemund og 
drak i
glæde
                  "),
("Konserves",549890031616,"Omfang

Pottemagerdrømmes
indlejrede planer
tager hånd om 
rejsens mål i
viljens krav til det
formbare

Et livslangt forsøg på
at vælge sin form
at rumme sit tilmålte
omfang
forvare og voksende
nynne klodens 
rytme

Over stepperne
blomstrer paletter 
svar til jagende 
eksistenser og 
visner kraft til
skjulte kilder




          "),
("Konserves",628206143490,"Omsorg

Vi ligger i
hinandens hænder
uden anden
beskyttelse end
den respekt vi
hver især finder
det for godt at
håndtere en
andens skæbne
med


26-12-2003
                  "),
("Konserves",68719476738,"Ord

For det meste er
det ikke
ord vi
savner
Hvad skal vi med
lyde når det er
en hånd vi
trænger til


                  "),
("Konserves",68727865344,"Orden

Djævlemasker griner
fra skæve søm der
skråt i væggen
holder fast i 
overfladens orden

Musselmalet porcelæn
på den stivede dug
skænket fuld af
hede

Solen tænder det
brogede tæppes
brand i blege øjne

Kultiveret bydes
kaos af bemalede
fade
          "),
("Konserves",1081344,"På toppen

Solen forvandler
vinen i glasset til
en magisk rubin der 
spejler verdens syv 
underværker 

Brændende grappa i
horisonten
          "),
("Konserves",16779264,"Påstand

Broen fastholder
sin påstand  

og lader sig
passere 

          "),
("Konserves",68761419840,"Pagten

Låste døre
vender ryggen til
gaden  og afviser
al kontakt
Kun en står
åben og skilter
velkommen der
går han ind  og
drejer nøglen der
optager ham  i
gadens pagt



          "),
("Konserves",17230200832,"Rejsefeber

Rejser gennem islandskaber 
fjernt fra centrum
forbeholdne besøg ved
grænser men aldrig
over
betragtning udefra inden
rejsen igen går af sted til 
egne der ikke kræver 
blottelse

længsel oversat til tryghed
på et ukendt sprog

          "),
("Konserves",41943040,"Ritual

Solen og skylaget
Det mælkede
vindue der
aldrig viser sig
gardinløst
Stilheden og
den forkrøblede plante i
karmen foran
rummet der holder
ud med den grå
dags gentagelse af
et ordløst ritual



          "),
("Konserves",0,"Sagn

En fugl 
balancerende på 
det blå lys på
kanten af et
skrig  et
råb gennem
tiden
Svæv over
hviskede sagn
der nægter at
dø men
tvinger sig 
vej gennem
afstødte
drømme
Rødder der
søger udspring
i intethedens
mulighed





          "),
("Konserves",0,"Sammenhæng

Det står i notesbøger med
forpustede bogstaver på 
mission
Det udfordrer brikker fra et
forlist ludospil fortabt i
memorering af
spillets gang
Bag tapeter flammer
glemte farver med
besked til en 
sammenhæng der   
holder vejret og 
ser op


          "),
("Konserves",64,"Skabe

Jeg lukker mine
skabe og lader
indholdet om at
komme til rette
Lyset står skarpt
gennem den
åbne dør og maler
tegn på de 
glatte låger 
          "),
("Konserves",138512697344,"Sorte vinger

Om lidt er det
for sent
Muligheden dør
mellem mine
hænder
Alligevel er
længslen der
rådvild uden
handlekraft
Englene har
sorte vinger

19. september 2003
          "),
("Konserves",343664492544,"Stien

Skovstien er 
bare vil
ingen steder 
hen blot bære
vore skridt der
uden mål
lever af den 
blide stilhed

19. september 2003

                  "),
("Konserves",2097152,"Storm

1
Stille

Stille tanke,
lad sindet følge
kroppens vej en
tid mens
brændingen falder
til ro

2
Storm

Du kender ikke
længere 
dig selv

Lyden af
styrtende mure
mod sprængte
fundamenter
skærer gule
flænger i den
blygrå himmel
over brændingen der
slår ind over dit sind 

Du græder men
ved det ikke
Lader dage følge
dage og planløse
skridt efter skridt gennem 
dit levede liv hvis
fragmenter
af mening  ikke
passer sammen

Endnu er du
ikke træt nok til
at modtage
forståelse

3
Ro

Brændingen falder til
ro under mit blik og
langsomt justeres
havets farve og 
himlens til den
ro der breder sig
indefra og henter
vished i landskabets
lange linjer der
sætter tingene på
plads  efter 
nattens kamp
med kaos

Skrigende måger fisker
opskyllede godbidder
mellem sammenbragte
elementer










          "),
("Konserves",69793220608,"Streger


Solen over
blodspor i
sneen
Streger 
tegner mønstre
gennem dyrs 
territorier og
skiller indenfor fra
udenfor i 
magtfulde strøg 
Sætter mørke for
og mørke 
bag når
hænder maler
tegn på
frie viljers
rygge


En rovfugls
skrig i
det blå
svæver bort 
gennem tiden



                  "),
("Konserves",1081344,"Svæv


På udbredte
vinger
svæver du fri
af smerten i
mit bryst
Skriget mod
den hvide 
dag snitter
kuldens panser
At være -
ikke på vej
over vinterbundne
vækster"),
("Konserves",268435456,"Svaner

Et sted flyver
svaner op fra 
en flade

åbner himlen for
lyset der flænger
natten i et
blodrødt kast

flyver  ind
mod mørket der 
ikke mere tør
håbe på dag
                  "),
("Konserves",550963775488,"Tårne


Blankpolerede tårne
over de hastende
skygger

Drømmeskrig mod
den anede frelse der
tavst lægger låg
over liv der 
selv må 
finde vej



4.sept.2003

          "),
("Konserves",549890041856,"Trækfugle

Vi er her på
vej
på tur i
evigheden der
lader livet
se med vore
øjne
høre med 
vore ører og
handle med
vore hænder


                  "),
("Konserves",1207959556,"Tvekamp

Umærkeligt forstærker
den sit greb og udmatter
langsomt sit offer der
før eller siden tvinges i
knæ for at indse sit 
nederlag til naturen 
der kun kan 
forliges ved at følge den
indre form der vokser på
tværs af idealer

Fremad i en
undergang der
kun kan fattes ved
at tage imod en gave
der ser ud som 
en dom




          "),
("Konserves",2048,"Tyst

Tyste øjeblikke lander på
en skulder og løser
uudsigelige ord

Et seende øje genererer
kraft 
Et andet gemmer en 
viljes skygge

Stilheden vil intet
vide og sidder tæt
          "),
("Konserves",270532608,"Under dække

Stjernerne var
der og månen
over havet med
drømmestien til
den tabte
horisont
Under mørket
mellem løse
tanker lange
solstrejf under
dække af den
sunkne kyst


                  "),
("Konserves",0,"Under stjernerne

Ikke i
ordene
ikke i de 
grænsesprængende
formuleringer
eller den nyskabende
syntaks

måske i forsøget på at 
vriste nuet af den 
banale virkelighed og
smage på
kraftens autenticitet

at indtage hverdagens virkelighed 
lagre det flygtige
udpege det upåviselige
lade læberne tage form af
det uudsigelige 

Stjernerne over
sejlende isfjelde i den
flammende polarnat
sætter andre egne i brand 
som Kleopatras kærlighed
der evigt styrter riger i grus

at rejse til verdens ende for
intet 

at skære igennem til
centrum og se 
man er hjemme


                  "),
("Konserves",17179869188,"Under trækronerne

Lysets udstilling af
forfald tager ikke med
fløjelshandsker på en
nøgen krop der ikke
vil kende sig selv men
kryber sammen bag
forhandling med den
stålsatte sandhed

Kendsgerninger forhandler ikke 
om værdighed eller
vilkår og vejen tilbage går
ikke her

Den nye dags spæde
blomst vokser frem under 
visne blade der
fløj i blæsten i går


          "),
("Konserves",274877906948,"Undervejs

Vi følger
gaderne gennem år
der ved bedre

Passerer skriften på
væggen og holder
hvil under træer der
drysser visne blade"),
("Konserves",0,"Vildskud

Uholdbare forbindelser i
opbrud
Lysende stjerneskud på
den knaldsorte
himmel
Månen står vel op
om lidt og smiler til
den vildfarne
vrimmel af løse
ender der ikke kan
samles

          "),
("Konserves",1073743872,"Vokseværk

Sandheden vokser på
sin gren
hopper rundt med
prøvende lyde og
basker plastiske
vinger der klør efter
frihedens svæv over
åbne spørgsmål der
ikke finder ro

          "),
("Konserves",68719476736,"Zombi

Og der står vi så
berøvet egne sanser 
Som  fladfisk på land snapper vi 
luft ad sære kanaler mens 
øjnene stirrer brustne uden at
forstå og ordene der 
ellers altid er rede til 
afsæt trykker sig på reden og
slår over i  uvante hulk af
mangel på manuskript 

Automatisk som
man plejer går turen til
ende


          "),
("Kun et øjeblik",16777216,"Æsken

Der er bund og
låg
Fire sider og et
indhold kun
jeg kender til

Jeg åbner
aldrig  men

nu og da
tager jeg den frem og
holder den i mine
hænder for at føle 
den ro jeg har givet
mig selv



November 2004
                  "),
("Kun et øjeblik",68719478784,"Åbenhjertig

Du har ikke noget imod at
dele din viden med
mig for den kan  kun
bruges til at
fastholde vores
balance

Stiger har
mange trin og det
gamle skab knytter  kun
sine skjulte 
skuffer




November 2004
          "),
("Kun et øjeblik",0,"Afklædt

Vintersol bag
afklædt natur 

Forsvarsløst  nøgne
flader

Himlen arret af
fugle i flok over

tages  opstyltede
ekko

Skyggers eftertænksomme
pauser og

kulden der finder det
sårbare sted




December 2004
                  "),
("Kun et øjeblik",268443648,"Bare det

Han gennemhullede
luften og sagde

Selv frøet der lander i
skyggen
forsøger at nå frem til
sin blomstring og
række livet frem mod
lyset

Han skrumpede og
sagde

Det var bare det



November 2004
                  "),
("Kun et øjeblik",275280568320,"Blade

Visne blade i
vinden 
Dage
lejret  i år
Langs hegnet et
spirende muldlag 
Knopper  løftet  ind i
lyset på 
smidige grene


November 2004



                  "),
("Kun et øjeblik",549755822080,"Blindgyde

Ingen kan standse
tiden
Alligevel kan livet
gå i stå og
konfontere én i en
blindgyde uden
håb om
flugt

Opad kunne man
måske forsøge



December 2004


                  "),
("Kun et øjeblik",0,"Brak

Han har 
mistet grebet,
tabt sagen på
gulvet og
ligger brak
Der er længere til
drøm end til
mulighed
Gennem et skår
kigger han skævt til
solen



November 2004
          "),
("Kun et øjeblik",1073744386,"Brand

Selv store bål
brænder ud og
blotter den
nøgne jord



November 2004
                  "),
("Kun et øjeblik",1048576,"Brand

Uden tøven 
springer jeg over
stakittet til den
glemte have og
genfinder straks den
mistede blomst 

Flammende glæde på
svajende stilke
stikker mit hjerte i
brand

Der er langt til
himlen men længere til
smerten og jeg er som en
fakkel der leder  
søgende på vej



November 2004
          "),
("Kun et øjeblik",549755822080,"Bror

Du knæler for
alteret og
kalder mig din
bror

Så vend dig og  
lad mig se dit 
ansigt
Jeg har jo ingen
anden og kender ham
på lyset i
hans blik





December 2004
                  "),
("Kun et øjeblik",269492224,"Bøn

Som en sølvfløjte i
min bambus et

magisk øjebliks
latter

Og fra det dybe

råbende stemmer til
lyset om ikke at
give op




November 2004
                  "),
("Kun et øjeblik",68753032192,"Comme il faut

Gemt bag hver sin
attitude er det
lige til at
holde ud og
Herre Gud -
Vi ved det jo
begge så
hvorfor lave om på
det



Oktober 2004
                  "),
("Kun et øjeblik",575659835392,"Dømt

Han løfter 
Han løfter dem og 
drikker
Han løfter hænderne og
fylder dem
Han løfter
dem til en
skål og
fylder dem
Han fylder dem med
livets gave  og
drikker 
Han løfter dem og 
drikker med en 
dødsdømts øjne
Han løfter hænderne til en
skål og fylder dem med
livets gave som han 
drikker med en 
dødsdømts øjne




Oktober 2004
          "),
("Kun et øjeblik",0,"Dansen

Superbia, Avaritia, Luxuria, Invidia, Gula, Ira, Accidia 

De syv søstre danser under 
prismelys i den 
gnistrende nat 
Tabte sko på 
isglatte trapper 
Blodspor i den 
nyfaldne sne 
Kometen på 
himlen varsler 
ufredsår mens de 
grå børn græder 

Snart skal 
maskerne falde 


Samhørende:
Før - Samling: Det ender med et smil
Nok -  Samling: : Det ender med et smil
Besøget - Samling: Hjertets rytme
Forlegenhedens rum - Samling: Hjertets rytme
Selv - Samling: Det ender med et smil
Dansen - Samling: Kun et øjeblik





December 2004



          "),
("Kun et øjeblik",13154385920,"Dug

Som dug på
ruden fortætter
dagen  sig  mod
natten
Med en finger
tegner jeg et
hjerte og  kigger op
på mørkets
første stjerne




December 2004
                  "),
("Kun et øjeblik",268443648,"Eksistens

I en lomme i
tiden
tapetseret med
vore drømme

Afskåret hvilende

Uden ende eller
begyndelse hæver
lyset mørket



December 2004
                  "),
("Kun et øjeblik",85920317440,"Elver

Uden ende eller
begyndelse 
står fortættelser frem
af mørket
I lange strøg løber
bløde udløb gennem
forsænkninger som
base for omrids af
substans og i
centret dit dunkle
øje og den hånd der
sender løfter og den der 
plukker roser hvor
ingen roser gror
Den sumpede jord
giver sig over til
åbent vand



Oktober 2004
                  "),
("Kun et øjeblik",8194,"Entre

Nuet er altid at
foretrække frem for
fortiden alene af den grund at
det er den eneste 
indgang til 
fremtiden



November 2004
                  "),
("Kun et øjeblik",68786585600,"Fejlskud

De havde tænkt sig det
skulle blive ligesom med
Adam og Eva men det
blev snarere som med
Kain og Abel -
og Paradiset  så de
aldrig noget til



Oktober 2004
                  "),
("Kun et øjeblik",50364928,"Fjern

Man kan komme så
langt væk fra sig selv at
man ikke længere kan
føle sine egne
behov og ikke  længere har
kontrol over 
spontane reaktioner

Så langt at man ikke
selv kan finde hjem




November 2004
                  "),
("Kun et øjeblik",8607236096,"Fugl

Hvis jeg kunne ville jeg da
godt prøve at være en fugl der
løfter sig fri af underlaget og
ser tingene fra vinkler jeg
kun har drømt om  og
er jeg stor nok vil jeg
svæve rundt  over byen og
til sidst forsvinde i en lille
sort plet på kanten af
himlen
Måske ville jeg smide en
enkelt fjer og se den
rotere ned og lægge sig
på græsset i det lille
anlæg ved kirken




November 2004
                  "),
("Kun et øjeblik",68719476736,"Gaden

Der er dem og
der er dem
De sidder som
født ved stivede
duge og  råber
upassende op på
banegårdscafeen
Det glimter fra
elegante smykker mens
de indfaldne munde
blotter  deres 
tomhed

Hvad vil vi dog her på 
de stivnede gader hvor 
solsorten synger
forgæves



December 2004
          "),
("Kun et øjeblik",16644,"Gylden

Solen finder gennem
årtier til dagen med den 
gyldne mur der 
stadig husker navnet bag
affaldstønden,
til lugten og
lyden af fluer der 
kaster sig ud af det 
blå foroven,
til spirende vemod fra
sprækker i furet cement og
skikkelser der prøver at
slå gennem tiden indtil 
stemmen kalder fra 
vinduet i skyggen skønt
fremmedgjorte mures 
fremmede navne 
ikke forstår



November 2004
                  "),
("Kun et øjeblik",16777728,"Heling

Vinden mod
huden og transparente billeder
foran den tidløse
dag der langsomt
modnes til
forståelse

Brikker der
omsider finder
anvendelse for sig
selv i en
afvist sammenhæng og
indser kompositionens
logik

Inden lyset kæntrer i 
nattens dugpunkt
falder det ind over
væggens mønster der 
løfter sig som det
vil ses


Oktober 2004




                  "),
("Kun et øjeblik",402653184,"Henslængt 

Grå dag kastet over den 
glødende sol 
Flænger i 
forhængets folder 

Over bakken og 
dybt ind i 
evigheden revner 
billedet i et 
blodrødt gab som 
et skrig mod 
mørket 




December 2004
                  "),
("Kun et øjeblik",68719478784,"Hvad så?

Tænk om man kunne
fjerne alle
ordene alle
attituderne og
de etablerede
manifester
Tænk om det  eneste der
betød noget var det man
faktisk gjorde
Ikke etiketter eller
overskrifter 
ikke umistelige
tilhørsforhold men de
handlinger man bevisligt
satte i værk og
kæmpede til ende
Tænk om det faktisk er sådan
og at alt det andet
er tomt teater der lige så
godt kunne afløses af 
folkedans eller poetry
slam i smileycafeer

Tænk og 
hvad så?




December 2004
          "),
("Kun et øjeblik",0,"Hvem

Hvem vil hjælpe de
fattige børn der
drager gennem
gader uden ende eller
begyndelse
Hvordan skal de nå
de kolde stjerner
hvis ingen
skærer gennem stilheden og
åbner virkeligheden på
en anden side
De nedrullede
gardiner viser
skyggebilleder bag
potteplanternes
jungleforsøg  
Nogen må vel vide at
de grå optog mangler
håb at kaste mod  
dørene 
Kun murene hører de
små fødders usalige
hvisken



December 2004
          "),
("Kun et øjeblik",549909995520,"Hvorfor

Gyldent land
Bæltblåt vand til
den anden side og
en fiskende terne der
vel ikke kender
hvorfor men 
snapper livet med 
alle sanser
En sky der 
lader sig drive 
hvorhen




November 2004
                  "),
("Kun et øjeblik",549755822080,"I hans billede

Når du ser menneskets
evne til
kærlighed
er det let at  forestille sig en
alkærlig Gud

Når du ser menneskets
evne til 
had
er det umuligt at forestille sig en
alkærlig  Gud

Når du mærker din
sult efter
kærlighed 
er det let at forestille sig at 
den kun kan  mættes af  
Gud



November 2004



November 2004
                  "),
("Kun et øjeblik",0,"I ordene

I ordene
ligger skjulte fossiler og
vogter deres viden
Engang var intet og af det
opstod det 
huskede og det
glemte
det synlige og det 
usynlige
det forsvundne og det
kommende
det sagte 
det usagte
det forståede det
ikke erkendte og
de krummer sig
sammen om 
kernen og bliver til 
sange der henter deres
tone fra
vindens leg i det
skabte

Gløder fra bålet
løfter sig ind i
mørket på vej mod
målet de kæmper for
at nå




Oktober 2004
          "),
("Kun et øjeblik",335544320,"Ikke min vilje

Og gennem mørket
kommer du til mig og
siger de ord jeg har
frygtet fordi det er dem
jeg vil have du
skal sige



November 2004
                  "),
("Kun et øjeblik",0,"Introvert

Endnu et bogstav
Så meget mere der
lader sig
tyde men selve
meningen står
langtfra klar

Eftertænksomt husker han
baglæns og opdager
nye sammenhænge men

er der overhovedet noget
at forstå





December 2004
          "),
("Kun et øjeblik",77376520192,"Katedral

Regndråbers
tæppebombardement
Lyse smæld mod den
øde gade og din
latter der 
løfter taget og 
bygger en
katedral af det 
grå




Oktober 2004
                  "),
("Kun et øjeblik",0,"Kattens

Og det var 
katten
Og det var
grøden
Og det var 
katten om
grøden

Og kun jeg ved
hvor varm 
den var
den grød



November 2004
          "),
("Kun et øjeblik",4,"Klip

Jeg åbner min
bog og finder
huller i bladene
Små partier er
klippet væk og 
hver gang jeg
åbner mangler der
mere og jeg
leder men 
finder hverken
udklip eller
saks og griber til
forsøg med
tilfældige stumper der 
ser ud til at passe på
den tomme
plads




November 2004
          "),
("Kun et øjeblik",68786585600,"Kom glad

Ikke nu men
når vi begge har
besejret vore
drømme


Oktober 2004
                  "),
("Kun et øjeblik",268435456,"Kontakt

Ordenes forening med
mørket
fletter en
rebstige for
tanken

Stilhed
fortættet i
nærvær




December 2004
                  "),
("Kun et øjeblik",16810496,"Krisen

Stort set
ingenting men alligevel en
uoverstigelighed

En sneskred på
en sommerdag

En bristet åre i
sindet

Og bag dit
billede ingen
horisont





November 2004
                  "),
("Kun et øjeblik",824633753600,"Kuling

Jeg gik ud i
efteråret og
blev til et
blad der gav slip og
lod sig  blæse
af sted i vinden

Højt under  og
igennem
men selv blade søger læ  og 
giver sig  hen



November 2004"),
("Kun et øjeblik",8192,"Kun et øjeblik

Jeg troede 
destinationen
ventede derude for
enden af mine
skridt
Måske tog jeg 
fejl af vejen og
målet
Jeg ved det om
kun et øjeblik


November 2004

                  "),
("Kun et øjeblik",566935724032,"Kurven

Kurven er pakket med
det hun har samlet og
skønt det er 
lidt  kommer hun dog ikke
med tomme hænder
Mest fylder smerten
så der er rigelig
plads til den tynde
glæde hun fandt
undervejs og håbet
der fylder en del 
Troen har hun aldrig
set men måske er
der tid endnu





November 2004
                  "),
("Kun et øjeblik",4429193216,"Liv

Det slanke skud 
svajende på 
balancen
Den massive styrkes
rå blok og
vækstlaget der
søgende tillader
forandring

En stjernenat en
solstorms uimodsigelighed

Vandrende kloder på vej mod
sig selv

Generationers ubrudte
kæde




November 2004
                  "),
("Kun et øjeblik",4563402752,"Luna

Måneøje over
afslørede sammenhænge
Forlegne skygger på den
blottede mur
Forsølvet blokering af
mørket 

Skyggemåne
høster sine spor
mellem nøgne
stammer

Morgenmåne
mellem jagede skyer
Forvildede forsvar
falder uden kamp for
nattens skorpioner




November 2004
                  "),
("Kun et øjeblik",150995456,"Lyst

Du gør det ikke af
nød men af en
lyst du ikke vil
kende selvom du
har slået døren op for
at give mørket hvad
mørkets er



Oktober 2004
                  "),
("Kun et øjeblik",274877906944,"Længsel

Længsel er vel
ingen synd men 
medfører en
smerte der
føles som straf
Med øret mod stammen
mærker jeg vibrationen af
liv der farer gennem 
vort øjeblik og din
uflyttelighed slår kreds om
lys der må
lade sig splitte i
øer
Dit omfang åbner sig for
den  der finder ind og 
slipper uden
spørgsmål 
Så stærkt du står på
roden har du råd til at
svaje for den skiftende
vind og lade kraften
søge sit  udløb
Og jeg ser en bille på 
vej mod lyset med
sig selv



 


November 2004
                  "),
("Kun et øjeblik",274877906944,"Mellemrum

Midt mellem
intet og  den
slidte gade sprænges
tanken  og
efterlader et
afgrundssug

Det berøvede træ
rækker efter
fugle i flugt




December 2004
                  "),
("Kun et øjeblik",549755822144,"Mit

Enhver tanke vender om foran
den lukkede dør der 
åbner sig når den vil og
kræver tomme 
hænder
Og det vil være som for
første gang så 
ingen ved om 
man kan bære håbet med eller 
om også det skal afleveres på 
uundgåelighedens tærskel

Der var engang en dør og
man gik ind og tog
alt det man 
kalder sit



November 2004
                  "),
("Kun et øjeblik",6710886400,"Morgenby

Det lysner
så snart 

Over tagene er
himlen i tvivl
Kun gadens bund har
ikke bemærket
dagens første 
spion der haster
forbi mens
kulden ruster sig til
det afgørende slag

Og højt over
træerne i parken
fastholder nattens
sidste stjerne sin
position


November 2004

                  "),
("Kun et øjeblik",6442450944,"Morgenhygge

Spurve i min
bambus 
Natten giver
slip og lader sig
tromle  af
morgentrafikken
Kun jeg
venter lidt inden
jeg stopper
hyggen i lommen og
kaster mig i 
dagen




December 2004
                  "),
("Kun et øjeblik",68719476736,"Muren

Da brevet gik ned som
en potent Berlinmur
stod alle vore løfter
på den forkerte
side
Lyset falder ind på
den arrede flade og
tegner virkeligheden
skarpt


Oktober 2004
          "),
("Kun et øjeblik",17196646400,"Muren

Midt gennem gaden
fremturer den 
forbudte mur
Han ved han er på
den forkerte side og
gør hvad han kan 
for at  gå som de
andre der vel har
indfødsret her
Det er en lettelse
når skyggen
slører




December 2004
          "),
("Kun et øjeblik",2147876864,"November

En hel spand
efterår  hældt ud
foran døren
Som en kongelig
person træder jeg ud på
den brune løber og 
går ind i den nyfødte
dag på sommerens
aflagte hylster

Og se, hvor vi
matcher hinanden



November 2004

                  "),
("Kun et øjeblik",135274496,"Nu

Hastigt og 
uden lyd, tanke eller
tøven
Livet tager og
livet giver men
dette nu er
mit 

Og det er
prisen værd




December 2004
                  "),
("Kun et øjeblik",2282749952,"Og

og glæden er 
som
en fjer der har
revet sig løs
af sin himmelske
ejer og 
er dalet  til
jorden hvor jeg 
finder den  under
den grå 
morgen  og 
sætter den i
håret og



Oktober 2004
                  "),
("Kun et øjeblik",0,"Omkostning

Det står foran mig og
kræver sin pris
Selvom jeg ikke er
én der betaler
roder jeg rundt til
der er nok i
sammenskrabede mønter



November 2004
          "),
("Kun et øjeblik",68719476736,"Overskredet

Som en brand mod
huden en ladet
nærhed for
tæt på utænkeligheden
af flugt
Lige for lige
gælder ikke her
udenfor grænsen



November 2004
          "),
("Kun et øjeblik",1048576,"På vej

Vi er altid på vej til
noget til bageren til arbejde til lægen
På vej til at blive rasende kørt ned af en
spritbilist eller til blive gift om
en uge
Med eller uden ben 
kørestol eller krykker har
vi fuld fart på mod næste måltid et
kys på kinden eller den middagslur
vi har ventet på siden i morges og
et eller andet sted over eller under os er solen
på vej over himlen eller på vej til at gå ned for at den kan blive klar til at være på vej op igen i
morgen
Det er spændende at være på vej for man
ved aldrig helt hvad det ender med og
bedst som man er kommet i mål med sin væren på
vej begynder nye på vejer at løbe af sted med en
på vej til noget man også skal nå frem til og selvom man sætter hælene i eller synes man bare ligger og sover suser på vej af sted med en ligesom med
solen der også er på vej uanset om det er lyst eller mørkt middag eller morgen




Oktober 2004
          "),
("Kun et øjeblik",17179872256,"Præcisering

Hans problem er 
for så vidt ikke at
han er villig til at 
sælge sin bedstemor men 
snarere at  han ikke har 
bedstemødre nok



November 2004
          "),
("Kun et øjeblik",17179872258,"Pyt

Er dette
Visdommens Hav
får ingen brug for
støvler



Oktober 2004
                  "),
("Kun et øjeblik",69860327424,"Radar

Og ligegyldigt hvad
du svarer har en
magt større end
min lagt en
lænke om mit
hjerte, dømt til at 
følge dig


December 2004
                  "),
("Kun et øjeblik",618659842560,"Så længe

De kommer nedefra som
en kradsen af
hårde negle
En insisterende råben der ikke
hører op
De kommer fra
baggrunden
En kraftfuld banken der
får os til at tale
henover og
le som fortætning af
vores  kreds 
De kommer
indefra og presser
på i
korte staccatolyde der
stønner sig vej
mellem ord

Så længe vi intet ved
Så længe vi intet har hørt
Så længe vi fastholder 
hinanden

kan intet ske




Oktober 2004
                  "),
("Kun et øjeblik",549772599296,"Sanseløs

Er det sådan at
vi tumler af sted som 
en stor skabning uden
bevidsthed om 
sig selv -
hvis organer og 
lemmer bekæmper hinanden uden 
forståelse for 
sammenhængen mens den
enkelte celle forsøget at 
blive stjerne for 
en dag ?
Er det sådan må man 
håbe på
eksistensen af et
kærligt væsen der 
våger over den 
sanseløse indtil den 
en dag kommer til 
bevidsthed og
bliver i stand til at 
tage vare på 
sig selv


November 2004


                  "),
("Kun et øjeblik",69860327424,"Snigløb

Ad en bagdør
stjæler jeg adgang til
dit hjerte og
sniger mig  frem til
det  inderste 
kammer,
fordrejer døren  og
slår mig ned

Du kender mig
ikke endnu men 
det kommer du til




November 2004
                  "),
("Kun et øjeblik",69810003968,"Spil

Når vi bliver bevidste om
vores eksistens har vi
allerede været her
længe og er i fuld gang med
at spille et spil hvor kortene er
delt om og reglerne lavet af
andre
De der ikke sidder i held må
se til på afstand sammen med
dem der har mistet lysten og
fordriver tiden som de nu kan
indtil de en efter en lusker af
uden at forstyrre spillet


Oktober 2004
         
Anvendt af Cafeteatret 2005 
i forbindelse med 
skuespillet \"Under bæltestedet\"
af Richard Dresser           "),
("Kun et øjeblik",68786585600,"Spor

Månen tegner en
sti over
havet og
bag os
ses  vore spor som
pletter på
flydende ild



Oktober 2004
                  "),
("Kun et øjeblik",8194,"Teater

Man kan ikke
være tilskuer til
sit eget liv for
enten lever man så
intenst at man intet
ser eller også iagttager man
så meget at man aldrig 
får levet



December 2004
                  "),
("Kun et øjeblik",17179869184,"Testamentet

Han satte sig på
hug og skrev i
sandet,
hviskede ord i
vinden og lod dem
stå,
så op og skrev
under med et
blik på
horisonten inden han
rejste sig og gik



Oktober 2004
          "),
("Kun et øjeblik",17314086916,"Tid

Så tæt på
vinter burde
ingen forsøge sig med en
blomstring 

Den blanke dag
svøber kulden tæt og
tegner en hårfin
skygge med
sol


Oktober 2004

                  "),
("Kun et øjeblik",0,"Tildelt

Ligeud for rødt og
grønt fra siden
Vente på gult og
se hvad der sker

Jeg kender svaret som en
clairvoyant der
selvsikkert tolker
skriften på væggen



November 2004"),
("Kun et øjeblik",0,"Tilfældigt

Der er ingen
løfter lagt ind
mellem disse
ord der forsøger at
holde sammen mod
deres opløsningen 
Ingen udkast
til reglementer eller
aftaler endsige
forklaringer
Kun mellemrum der
adskiller tilfældige
ords tilløb til
mening



December 2004
                  "),
("Kun et øjeblik",68769841152,"Udenfor 

Så tæt på bliver
vægten af det aldrig
ejede for
tung

Skyggen søger mod
mørket og
skjuler mine
skridt



November 2004
          "),
("Kun et øjeblik",4563402752,"Under 

Ladet stilstand.  Lyset 
frosset i en 
stivnet attitude 
Undertrykte lyde 
rettet indad som 
eftertænksomhed 
Mørket der 
intet ved om 
solglimts latter i 
den udfoldede dag og 
kuldens snærende 
greb 
Under huden og det 
tilsyneladende 
Under den lammede 
tanke 
I nattens gnistrende 
stilhed 
Bløde vinger letter i 
mørket og 
fuldfører deres bane 
før dag 




December 2004
                  "),
("Kun et øjeblik",268435456,"Under taget

Under taget på
huset i haven har
spurven fundet læ
bag mørket
Og nedenunder i
lyset finder et par
øjne sig selv i
ruden og fatter om
mørkets magt


December 2004


                  "),
("Kun et øjeblik",85916131328,"Univers

Ikke det du
ser
Ikke det du
hører
men mere
Bag skærmen et
univers
levende
følende
søgende




November 2004
                  "),
("Kun et øjeblik",68753031168,"Værtinden

Hun har alt
parat som hun
synes det skal
være

Aftenens forløb
har hun flettet ind mellem
de to kuverter og
over bordet har hun
hængt sit håb der 
matcher dugen og de
foldede servietter




Oktober 2004
          "),
("Kun et øjeblik",68736257024,"Vendespil

Sort på den 
ene side,
hvid på den
anden og 
ganske ferm til at 
vende den rigtige 
side til



November 2004
          "),
("Kun et øjeblik",206208794624,"Vi

Mellem de
legitime kender vi
ikke os selv

Længsel smerter som
sult

I sprækken mellem
fortovsfliser 
modner en plante 
sine frø



December 2004
          "),
("Kun et øjeblik",68803887104,"Vinger

Baflelyde drysser fra
sorte vinger

Også jeg er en
flygtning men
gemmer mig bag
solstrejf i dit
hår


Oktober 2004
                  "),
("Kun et øjeblik",0,"Vækst

Under fladen i 
mørket 
Under det
blanke spejl
I skovbunden under 
bladdækket
Anemoneblade over
den  visne sommers
rødder 
Tankekæder vuggende i
mørket  
Fortætning mod 
bunden
Orme og en
pludselig ændren retning
som en tangsnares
grænsesprængende
spring
Mellem lagene
lyttende pauser
Svangre ideers uafvendelige
vækst
Årsagskæder 
uberørte af tankens
kontrol
Naturs ustoppelige
vækst gennem
uigennemsigtigheden
Perlende latter fra
et ubekymret barn







December 2004
                  "),
("Kun et øjeblik",95563123714,"Værdiansættelse

Ofte viser vi
større omhu i
omgangen med ting end
med mennesker -
men de har jo
også mulighed for at
være her når 
vi er væk



December 2004
                  "),
("Lys i mørket",695818258432,"Bortvendt

Under den blinde
himmels bly
sanker de sorg
af frosne marker 

Sanker sorgens
kolde høst 

Fravendt lyset
gråner skygger
mures sønderskudte 
skjold

Uden længsel
ingen smerte
Uden sæd ingen 
høst

Sorgens bitre
vinter 
knuger dit 
bortvendte
bryst



                  "),
("Lys i mørket",8725200896,"Drift


Jeg sanser
bjørnen i
dens hi
svøbt i
flammenatten
Stjernehobes 
tågeslør over
dvaske biers
summen
I skarpe skygger
vokser lyset
frem af 
væksters drift
Solhjul over
stejle bjerge
Pollenskyers 
lyse sæd
Urkraft emmer
under dække
af mit
vintertrange
bryst

          "),
("Lys i mørket",8993636616,"Dåb


Solskakter sprænger
ladens mørke
og forgylder en
dynge korn
Støvet udspændt
i lysgalakser
svæver over
børn der
bader i
regnkaskader af 
skrå universers
gyldne sæd

Mørkets lyse
latter


                  "),
("Lys i mørket",13154418688,"Fakkelbærer


Skridt gennem
mørket
Lyst i tanken
Tomme gaders
mørke skakter
gløder i min
tankes lys
Fakkelbærer
gemt i mørket
Lyse tankers
skridt mod
mørke gaders
sten

                  "),
("Lys i mørket",833626308612,"Fastholdt


Mørket
hænger fast
i dagen
og holder den 
bundet tæt 
over jorden

Nøgne grene
strækker
slanke knopper
ind i lysets
vilje


                  "),
("Lys i mørket",550193070096,"Kaste kim


Baners fastlagte 
præcision 
Frøs vilje til
at lade sig så, 
gå opløsningens 
vækst i møde
underkastet
celleprocessers 
styrede forløb 
Blomstre ind i
intetheden som
forudsætning for kaldets
evighed
Kaste kim i 
mørket

                  "),
("Lys i mørket",554336026625,"Kigger


Mørket presser sig
tæt mod
huset og glor
med sorte
glugger
Jeg stirrer
forbitret igen
men ser kun
mig selv i
tomhedens
lurende øje

                  "),
("Lys i mørket",4564582400,"Læ


Bag tagene 
brænder himlen 
rester af 
dag mens 
mørket letter på 
lange skygger
Af tænkte tanker
sættes spæde
skud der
vokser frem
i læ for
lysets skarpe
vinde


                  "),
("Lys i mørket",550025297920,"Lys i mørket

Jorden stivnet under
frosten der har 
bidt sig fast med 
stive kæber

På lysets autoritet
springer livet ud
og rækker tillid frem på
tynde stilke der
ikke lader sig 
sige imod

Bag lange skygger
står den lave
sol og lover andre
tider inden den 
forsvinder for at 
tænde lys i
mørket nedenfor


24. februar 2004


                  "),
("Lys i mørket",26039288072,"Lysalf


Mørket ruger
hårdt på 
dagen
før det vælger
lange vingers
flugt
Et frisat solglimt
tindrer i 
en lille drengs 
latter gemt i
fjerne somres
hede
Han står i lys til 
knæene i 
fjordens lune 
vand og
fanger solens
smil med 
våde hænder
Gemmer lys
i glæde




                  "),
("Lys i mørket",2147483648,"Morgen


Dagen står 
klar med 
nybrændt lys
friske lyde og
ubrugte minutter 
til distribution
Gaderne gaber af 
tomhed og 
griber de første 
bilister og 
sender dem væk 
fra sig selv i 
lige rækker der 
strækker sig
vej gennem 
timernes invasion
Skridt over fliser
udfordrer tiden der 
sindigt gumler på 
liv og lytter til 
solsortesang
fra haver gemt væk 
bag ved huse der
råber på ro


          "),
("Lys i mørket",558748401664,"Nødvendighed

Flygtige toners
lysspor gennem 
tågen

Piblende strømme
af liv under
sneen baner sig 
vej ad
nødvendighedens 
kanaler og 
løber sammen i
lysets vilje.

Trækkende
gæs over fyret
på længselsbaner
i drift



                  "),
("Lys i mørket",275280560132,"Pilegrene


Strukturers vækst
i forløb
der fører mod
en grå stivnen
stærkere end
farvens protest
der flammer
lakrødt gennem
besindigheden

Furet bark
omslutter
livet der
løftes mod
branden i
lyset



                  "),
("Lys i mørket",6853492736,"Ruder


I timen før dag
står mørket tæt
om husene
Blinde ruder
venter på at 
pakke morgenen 
ud og tænde
dagens lyde
før lyset
lander på tagene
og lader 
mulighederne gro



                  "),
("Lys i mørket",558749712384,"Snedråber


Snedråber 
på vej fra den
frosne muld
Spinkle klokker 
kimer lyset 
frem af 
vintersind i 
mørke
Vækstens tillidsspring
gennem kuldens
panser åbner
livets vandring ind
i lyset


                  "),
("Lys i mørket",4563402752,"Solsortebyen


Byen sover
bag mørke 
gader
Belyste butiksruder
gaber
uden grund
Skyggefugle
afsøger 
tomheden

                  "),
("Lys i mørket",135266304,"Svøb


Sneen mod
det dækkede
land
Hudens sårbare 
liv under
åbnede sanser
Nyfødte rækker
flader lyset
liv
Under svøb
ånder fostre
drømme i
mørket

          "),
("Lys i mørket",4564451328,"Svanenat


Svanevinger 
hviner den 
blå dags kulde ind
i natten og 
tænder ulmende 
længsel i
horisonten over 
vingeløses håb om
flugt ud af 
tankens 
fængsel

                  "),
("Lys i mørket",11006902272,"Syvsover


Solen står
op for sent
til at se
min stille
morgenglæde
der oplyser
vinterens
mørkeste dag
med ren
forventning om
lysets vækst
når solen
letter fra
reden

                  "),
("Lys i mørket",554453434368,"Tegn


Tomme horisonter
åbner sig for 
solens opgang i 
nyfødte drømme
og lukker efter 
dagen der kun 
forglemmende tog mod 
lysets kald til 
blomstring

Vinger maler
tegn på
den natkolde
himmel

                  "),
("Lys i mørket",554320265216,"Vækst


Citrongult
skilles
mørkets felter

mørkehimmel
mørkeland

nattens rejse
mod dag

mæt af
mørket
vokser lyset
i mit
hjerte

                  "),
("Lys i mørket",73282879488,"Vagtskifte


Tusmørkets fred
løber gennem 
kroppen som et lag 
beskyttende olie
Dagens lyde folder
sig sammen
mens stilheden pakker
ud og rejser sin 
kuppel besat med 
flygtende mælkeveje
Under stjerner svæver
vore skridt efter 
tankers frie leg 
i tid


                  "),
("Månelys",152051840,"Betragter

Lad græsset feje 
himlen ren; dagen 
står som en 
fakkel foran mit 
ansigt
En myre bestiger min 
hånd og løber som 
efter et mål
Øjet fanger en
lærke der synger sit 
eget sprog
Jeg er




Juni 2007
                  "),
("Månelys",8606711808,"Brand

Vore ord er som
bålets flakkende
genskær på
mure
Tankerne flammer
fra den kerne ingen
kender; end ikke
den der tror at
kende sig selv



December 2006"),
("Månelys",9680463360,"Bærerne

Vi går aldrig
alene men 
sammen med dem
vi lod bygge med på
vores skæbne og
tættest går dem vi 
ikke kan slippe fordi de 
bærer de sider af 
os selv vi 
ikke vil
kendes ved



December 2006"),
("Månelys",8590098432,"Den grønne måne

Hun danser i 
sneen i 
de røde sko
Hun danser 
under den grønne
måne og
sætter spor i 
ubetrådte lag
Kun én har danset 
vil de sige i
morgen og studere
sporene efter
pigen i de røde sko
men ingen vil læse
hvad hun bar i 
sit hjerte og 
månen er ikke den der
røber hvad den
så i 
hendes øjne


Marts 2008
                  "),
("Månelys",8606753280,"Desperation

Som hængende  i
neglene over en
afgrund
klamrer jeg mig til
dagene 
til timerne
minutterne
til bare et enkelt
sekund
til det sidste 
til det allersidste
i håbet om at
tiden vil åbne sig
og hale mig
ind
sætte fast grund under
mine fødder og
lade mig gå



November 2006

                  "),
("Månelys",26977797632,"Determineret

Hun går ned ad
gaden som én der
bærer en beslutning
Havde mennesket hove
ville hun sparke bagud
og sende byger af
forurettelse mod 
hvem hun kom forbi



August 2007"),
("Månelys",558481014816,"Dun

Fuglen flyver over
havet
fanget i sin banes
logik
Hiroshimas silhuetter
tøver i tiden 
end  ikke et svævende
dun går tabt men
falder på plads som
en brik i et 
puslespil der finder 
billedets rytme og
fuldender alt


August 2008

                  "),
("Månelys",9714016768,"Evaluering

Hvorfor byggede jeg
forsvarsværker mod
andre, når
angrebet kom fra
mig selv


Maj 2007"),
("Månelys",8590107152,"Flygtig

Én valmue mellem
markens kamiller
som en dråbe blod
på et lagen der
ikke mere er som
det var for hvem ser
andet end
dette vidne der
forvandler fladen til
ramme

Én valmue på
den bølgede enheds duft
skyggen af en
fugl farer over


Juli 2007"),
("Månelys",134226448,"Forløsning

Regnen finder vej og 
løsner
Jorden tager imod og
omsætter
Selv denne 
smerte lader sig
undergrave og
befri
Der vokser valmuer i 
mit spor som
jordens svar på 
mit spørgsmål


Juni 2007"),
("Månelys",8589935106,"Forsikring

At blive hængende i
smerten over dit
livs fejltagelser,
beskytter dig ikke mod
at begå nye


Maj 2007"),
("Månelys",69843558400,"Forsvar

Du invaderer mit
rum og jeg 
driver dig ud med
stokkeslag
Et menneske må
have et sted der kun
er tilgængeligt for
det selv og dets
skæbne



Maj 2008"),
("Månelys",50373120,"Fosterhinder

Mellem himlen og havet
fosterhinders silkemembran
Mågeskrig saksende
dagens rand
Bag mit spørgsmål et
menneske
klippet af sin
baggrund,
en oprakt finger i det
stædigt blå



Januar 2009"),
("Månelys",8757747712,"Fri

Han er der
ikke
Fra sit sted bag
tiden har han
lagt en føler ind der
hurtigt kan
kalde ham tilbage


Februar 2007
                  "),
("Månelys",268446208,"Frihed

Du ved vel hvad
du gør,
lyset har lange arme men
bag mørket gemmer sig
større dybder end
selv lyset kan nå



April 2007"),
("Månelys",8641093664,"Frost

Sådan dufter
fyr
Sådan synger fuglen
foran døden
Jeg går med mit over
den  frosne jord der
låner mig min
krop
Jeg sætter mine
spor


Marts 2007"),
("Månelys",2417492608,"Genfødt

Lyset åbner døren til
den nyfødte dag og
som en overlevende
træder jeg tilbage til
livet;
over mit
hoved skriger en
måge sin glæde der
som en skalpel
blotlægger 
skabelsens kærlighed
Øjeblikkets evighed
bærer mine skridt


April 2007

                  "),
("Månelys",8623522304,"Getsemane

Angsten ligger på
puden som et
frø¸ der 
afventer natten som
lader det spire og
danne rødder der
omslutter hjertet
Højt som et
træ springer væksten
i myrriader af 
stjerner
der en efter en
styrter sig gennem
tanken og afsætter
spor der fører til
intet og kun gør
mørket end mere 
mørkt


August 2007

                  "),
("Månelys",163376652324,"Går du

Du stønner af smerte
og ser på os,
ængstelig
for vores reaktion
Kan vi klare
din smerte

og så skal du 
på toilettet,
den transportable toiletspand
som hjemmeplejen har
sørget for
Stuen er nøgen
og din blufærdighed
større
end smerten
Vi går
mens far forsøger
at hjælpe dig

Du spiser lidt,
en gestus
Så lægger du dig
tungt på siden
og prøver
at flytte benet
så det ligger
bekvemt
Du er ikke helt klar
et øjeblik
Siger noget
du selv kan høre
er tågesnak
og smiler forlegent
Ja nu må vi se
hvordan det går
med det hele


I ved jo nok
det ikke bliver bedre
med mig
Men hvornår det så kan blive
ved jeg ikke
Det skal I
ikke
gå og tænke på


Du husker ikke
de 20 elektrochok
du fik
under tvang
Dine klager
Fantasierne
om gift,
levende brænding
Din endeløse vandring
ad den lange gang
fra dør til dør
Du husker ikke
at ingen hjalp dig
Det, husker du ikke
og vi fortæller dig
ikke alt

Den er ikke rigtig sivet ind
siger du,
dødsdommen
du fik af
den sympatiske læge
og jeg forstår
pludselig ikke
hvad vi er her for
Stjernehimlens
uendelighed
og dit
Sarajevo
smadrer mit
filosofiske
verdensbillede
der ikke
kan hamle op
med din virkelighed

Du skal dø¸
og du ved det
Alt fortæller dig det
Din krops
smerter og 
svigtende funktioner
Lægerne
Vores vagtsomhed og 
ændrede adfærd
Spiser du
overhovedet
noget

Og hvad andet
kan jeg gøre end at
gå, da livet kalder


Marts 1996
Februar 2008
                  "),
("Månelys",21793645056,"Ham

Natten åbner døren til
sit mørke rum og
hun går ind fordi 
hun ved at
vejen ud går her
igennem 
Hun møder kun sig selv i
nattens spejlkabinet og
hører glemte lydklip af
sin egen stemme,
scener fra livet
skriger  fra
skjulte skærme og 
mens  hun løber gennem 
dommens filter 
ved hun at det der 
bliver tilbage til 
morgenens lys
er et tomt billede uden
sjæl,  som en 
slanges afkastede ham



Marts 2007"),
("Månelys",8590721024,"Havis 

Edderfugle bag
tågens kald
kom nu
kom nu
et strejf af lys der
spidder vinterens
hjerte



Januar 2009"),
("Månelys",8740970496,"Hegn

Vilde heste løber over
engen 
Intet vil jeg, kun se
Vinden mod huden
husker
Et kæntret øje spejler
liv i  balance over et 
fejltrins pris 
Vilde heste løber
under viljeløse
skyer


November 2006"),
("Månelys",25820241920,"Hjem

Bakkerne runder under
himlen men det er ikke
mine bakke
Træet der rasler sit
eventyr i vinden er ikke
mit træ og jeg 
er ikke mig mellem
disse mennesker men
en ventesal for
travle rejsende der
tager ophold et øjeblik
inden de samler deres
bagage og rejser
videre til det sted der
folder dem ud og
lægger dem på plads i
et mønster der
giver dem mening og
sammenhæng


Juli 2007
                  "),
("Månelys",8673863168,"Hjertemagt

Æblekinder så
røde
Himlen så
blå
Hvad vil du mere
hjerte?
Alt det et
hjerte kan få?
Men ved du, hjerte,
der gives steder
end ikke et hjerte 
kan nå?
                  "),
("Månelys",558630961664,"Identitet

Man skal ikke
forveksle sig selv med
mørket og 
heller ikke med
lyset
Mennesker går på
jorden sammen med
mennesker fordi de
skal samme vej



December 2006"),
("Månelys",554319478784,"Julenat

Barnet sover med
jordens lyde
Stjernen  over 
natten
Øjne skal se og
Ører høre men 
først skal mørket 
gøre sit



Januar 2007"),
("Månelys",8589944832,"Klip

De vågne 
deler skæbne med
de sovende
De dødes handlinger
lever endnu og
blander sig med 
drømmene hvoraf
fremtiden klippes
som billeder af
et ark papir



November 2006"),
("Månelys",8657092624,"Kløver

Husker du duften af
kløver
de lange dage der
aldrig fandt ro under den
splintrede himmel
et hastigt kys under
drivende skyer
de fire blade der
glemte deres løfte



August 2006"),
("Månelys",25786589184,"Kunstner

Han malede en
cirkel over
sit hjerte
Rød cirkel med
hjerte
Rød cirkel med et
bankende hjerte som
stilhedens 
forudsætning
Hvem banker et
hjerte for
Hvem banker et
hjerte for et helt liv mellem 
fødsel og død
Han skrev sit navn i
højre side som en der
underskriver en
check



April 2007"),
("Månelys",68786618370,"Kærlighed

Kærlighed kan ikke
erobres,
kun skænkes


December 2006"),
("Månelys",94657052676,"Langs åen

I går gennem 
regnen og 
jeg hører jeres
latter som klokker af 
sølv fra 
poplerne langs 
åen 
I går på fødder uden 
byrde mellem alle
menneskets valg
Og det er ikke min tid
I er som valmuer i 
kornet på de 
våde marker og 
jeg følger jer i 
tanken ad den 
kendte sti og 
mens I går lader I 
væden skylle 
tvivlen af jer og 
endnu inden vinden har 
taget jeres 
stemmer har I 
glemt



februar 2007
                  "),
("Månelys",283603369988,"Livstræ

Tiden spændt ud
mellem nøgne
grene
Fremtiden skåret  i
mosaik 
Gennem det levede
netværk kanaler for
kraftens vej til 
vækst


Marts 2007"),
("Månelys",549890031652,"Lommer

Jeg tæller tid,
samler på sekunder og
nøgne øjeblikke
Gemmer overskud til
travle dage og
udnytter spildtid jeg
finder rundt omkring
Jeg vælter mig i tid,
fylder mig med
stunder ingen vil have og
har til hensigt at 
sprænge grænsen mellem
mit og uendeligheden
Jeg har lommerne fulde af
overset tid der 
ligger parat til at
blive sat ind ved
fronten


Februar 2008
                  "),
("Månelys",10739646464,"Morgenrødme

Solen fletter sine
stråler foran den
skaldede måne
Havet støbt i
nuets inspiration
Spor i duggen efter
en der gik


10. december 2006"),
("Månelys",2553315456,"Morgensølv

Sølv er
dagen, 
svøbt om
sit indhold

Havet, en 
svævende hinde 


Vi går i
lys på
kanten af en
ternes skrig og
lader os fylde med
evighed

Sølv er dagen 
og et øjeblik endnu
husker sandet
vore spor


April 2007"),
("Månelys",563111559168,"Månelys

Mosset tager
lyden på sig og
lader os svæve
Kun månen kaster
en anelse om
liv  ind i dette
mørke der kun vil
sig selv og sit
eget
Vi leder stadig og
giver ikke op men
følger sporet af
sølv der 
ved om det
levende lys



Januar 2007"),
("Månelys",644447502352,"Nadver

Hun sprang ordene over
og lagde sin
kærlighed i ovnen
Hun plukkede en
blomst af den 
vinterangste have og 
satte den i 
vasen midt på
bordet
Så sad de der i
hendes stilhed og 
delte det hun havde
Og det var nok


August 2007"),
("Månelys",69860360704,"Negl

Som en
tappende finger mod
en rude
En hård
insisterende negl
Revner jeg,
skærer du dig og
blotter dit
røde blod



Marts 2007"),
("Månelys",262144,"Nyt år

Det nye år står
frem som en 
fremmed man lige har 
mødt
inden de første
ord,  hvor man  
endnu forholder sig
prøvende til 
hinanden uden at
kende den 
andens fortid
eller den 
bagage begge
har bragt med



Januar 2007"),
("Månelys",150425600544,"Nærhed

Dine øjne gemmer sig
blandt birkens blade,
dit smil i de 
vandrende skygger
I nattens sorteste
time kommer du
til mig som et
svalt kærtegn der 
lægger sig
omkring mig
Dine øjne ser mere
end mine og
dit smil er på vej
til en stille latter
fordi du ved at
afstand aldrig
skiller, 
end ikke denne der 
nægter mig ro

Maj 2007"),
("Månelys",549890039808,"Nærvær

Vi tigger under
kolde stjerner 
uden at ænse
jordens sang



September 2007"),
("Månelys",25820176384,"Nøgle

Ordene
bandt hun sammen i
en lang tråd som
hun vandt om sin
længsel
Nu og da
rullede hun et
stykke ud men 
aldrig nok til at
blotte den
skjulte kerne


November 2006"),
("Månelys",1124106240,"Nøglebærer

Dit had er som
en lås der
blokerer dig
indtil du selv
låser den op


April 2007"),
("Månelys",68820181504,"Overlæg

Aftryk i tiden
Afstødte celler af
liv der var 
Din hånd, 
varm mod min der
ikke tør

Lyden af et ur
der ikke ved
bedre


Februar 2008


                  "),
("Månelys",3586,"Paradoks

Begavede mennesker
træffer ikke nødvendigvis
begavede valg


Februar 2007
                  "),
("Månelys",10758135936,"På kanten 

af  det vældige landskab
formet af klodens
åndedræt

Dernede

en strømmende å 
følende sig  vej mod det
store hjertes
slag 

Så tæt

en dugdråbe i den
skære morgen fanger 
livet og lader sig 
falde

Se

den sorte fugl
bringer føde til reden

Her står jeg med
mit



maj 2009"),
("Månelys",8607891456,"Regn

Gaden 
flammer farver af 
det grå, som 
en uventet blomstring af 
en overset


November 2006"),
("Månelys",1057824,"Ro på

Hvis døden er
det endelige mål, som
fødslen er begyndelsen,
kommer ingen for sent og 
ingen behøver at
bekymre sig om at 
finde frem


Januar 2009

                  "),
("Månelys",627249782784,"Sandkorn

Lejret dybt i 
milens tro
denne splint af
evighed, alene 
mellem myriader, tæt
i længsel efter 
enhed under 
himles tegn 

Hvem kender styrken
af sin  egen tro,
af sin egen vilje til
forening

Vinden blæser som
den vil og lader sig
udfordre  som
skæbne

Februar 2007

                  "),
("Månelys",558766268932,"Skabelsen

Livet lærte mig aldrig at
fjerne angsten
men at bære den
i min fornuft og
min tro på livets
styrke
Livet lærte mig aldrig at
finde fred i mig
selv men i troen på
livets egen evne til at
finde vej
Jeg lærte aldrig at
tage mod livets gave
uden bekymring
men glæden over
skabelsens gentagne under
finder mig
stædigt som var det
af betydning at
netop jeg er mig
 



September 2007"),
("Månelys",549891866624,"Skabelsens ord

Bladene kantet med
intet,
fugleflokkens faldende
spiral
Et vindpusts lydløse
høst af rigdom,
betydningsfuldt
ikke som  tegn men
som  ytring i en
dialog mellem
elskende


Marts 2009
                  "),
("Månelys",134750240,"Skitse

En fugl
fanget i 
et glimt i 
et øje
Udenfor tiden
angsten på
klaprende vinger
Livet tegnet på
det nøgne lærred
med lange 
lette streger


Juni 2007
                  "),
("Månelys",146432631424,"Skyggesorg

Skyggen siver ud af 
muren og breder sig over 
jorden som blod der 
farver smerte og 
trænger ned og 
overlader klodens liv 
det,
der ikke mere er  og
ikke mere kan leve i
lyset
Skyggen tegner sin
skarpe grænse på
den lysende dag men
overskrider aldrig sin
beføjelse, den
vokser imod
Skyggen er blå og
jorden er åben mens
sorgen giver sig
over til klodens
magt
Selv mørket er kun
en kåbe der for et
øjeblik skærmer
den levende dag



September 2007"),
("Månelys",137574261248,"Smertens styrke

Smerten ved at
miste, er
beviset på at
du har ejet


December 2008"),
("Månelys",575542403072,"Solglimt

Hun går i
solen og
fletter stråler i
sit hår
Hun lader vinden
forme et
ansigt mens
hun beder til sin
Gud og taler til
mennesker
Hun ser uden
vurdering og
lytter uden dom
Hun er som et blad
der fanger lyset i
glimt og lader sig
høste af jorden
når væksten er
slut



September 2007"),
("Månelys",135266304,"Som

Som en flamme
der tændes i
mørket og
lyser sin tid
Som en duft der
stjæles af vinden
er håbet der
lander et øjeblik i
det vi troede dødt
Som en hånd der
rækker over
ligegyldigheden og 
planter liv

Som en sommerfugls flugt over
lunefulde bølger
                  

December 2007"),
("Månelys",8725766144,"Som nu

Solsortesang
Slanke toner
bag fremmede hække
En kat lister gennem
et tomrum og
finder ud mellem
skygger som en
tanke, der
dukker op og
forsvinder igen, uden at 
afsætte spor
Lange lyse lyde
skal fylde mit nu
det øjeblik det 
tager at føle lykke





Juni 2008"),
("Månelys",786432,"Sommer

Solsorten synger i
haven
Vinden driver dufte ind
gennem døren
Dagene hænger på
snoren og 
dasker


Juni 2008"),
("Månelys",1073778178,"Staldtips

Ingen kommer forbi den
der løber fra sit
ansvar

November 2007"),
("Månelys",8607770624,"Status

Det er denne
egoisme
Det er dette had
Det er denne
foragt for 
liv der
skal være
vækstlag for
kærligheden
Det er disse
hjerter af sten der
skal huse håbet og
troen på en
fremtid 
Det er  med disse hænder
vi skal sikre livet for
hinanden


Juli 2008
                  "),
("Månelys",4295000064,"Stjerne

Stilheden hængt ud på
månens horn
Nær horisonten en
stjernes insisteren
Ord dræber det der 
kun kan siges uden


November 2006



                  "),
("Månelys",8724678656,"Strandskaden

Lyset hugger
kanter af
den bleg dis
Røgfanen ud af 
himlen udpeges af
fingeren af
stål der præcist
finder det ømme
punkt
Det flade tag
oversættes til
strandbred af den
kæmpende fugl


April 2008"),
("Månelys",619666483200,"Synd

Der er kun den
synd du
begår mod
mennesker for
vejen går  altid
gennem den
anden
Guds kærlighed skal
du ikke jage, den 
finder dig nok


Marts 2006"),
("Månelys",549772599808,"Tankespind

Tænk om dette er
et skuespil der er på 
plakaten præcis så
længe at alle
medvirkende prøver 
at gennemspille 
samtlige roller



Januar 2007"),
("Månelys",560765379072,"Tegn

Er det havet eller
smerten,
den blodrøde flænge i
himlen eller
skriget

Urørlige fugle 
tegner en
formation på
lyset som et
varsel der kan
tydes,
et budskab fra en
magt udenfor 
rækkevidde
Hvad blev der af
tilliden,
den faste tro på
livets grundlag

Mørket er
tæt over jorden,
kun himlen bærer
spor af den
kommende dag



Oktober 2007"),
("Månelys",11006386304,"Tema 

Dagen pibler af den
sorte fugls næb
Splintrede tilfældigheder
søger sammen i
teorier og gør krav på
helhed
Brudte linjer finder hinanden i
perspektiver der
søger deres fortsættelse og
ryster en horisont på
plads 
Historien skriver sin
historie og mens 
lyset vokser rejser troen sin
hvælving over de
søndrede liv
Fuglesang med ukendt tema



Januar 2008"),
("Månelys",78383188482,"Timing

Den smerte vi
flygter fra ved ikke at
se og høre mens
tid er, vender tilbage
uden den nåde det er
at kunne handle i
situationen


December 2006
                  "),
("Månelys",50331680,"Tomt

Rummets afmagt
Lyset der trænger ind og
jagter  felter på
lyttende vægge 
Skabet der
giver lyd som
det skamløse gulvbræt 
Lugten på flugt gennem et
vindue



Januar 2007"),
("Månelys",94606763520,"Troldspejl

Han kysser sig og
kærtegner sit
billede i
hendes øjne
Han vender sig mod
spejlene og ser
intet  andet
Der findes intet
bagved,
kun ham og det
udvalgte objekt, for 
endnu har ingen
kastet den
første sten


Juni 2007"),
("Månelys",13153337408,"Tærskel

Døren er
midt på
engen

på græs mellem hvide blomster
Og svaler flyver igennem
og natten lader
månen finde vej
og kaste en 
lysende firkant  
fra sig

Vore skygger vokser sig lange
og finder vej til
mørket



August 2007"),
("Månelys",9848758304,"Uglen

Uglen sidder i
træet
Træet forholder sig
roligt
Jorden er rede og
over himlen breder sig
det skær af accept 
der gør det muligt 
at lette fra sin
position og uden
lyde blive til 
målrettet skygge
Jorden er rede og
de dernede ved af
intet før natten 
skifter side
Hvem ved hvad der endnu
kan ske før dagen tænder
bag havet
Træet er rede og
jorden forholder sig
roligt under fuglen der
gør hvad den må for
at høste livet af
skaberens hænder
Vi sår og vi høster og
fravrister livet  det liv
vi skal leve for at
følge planen vi 
ikke kender
End ikke vor egen
rolle afslører sig for os
men lader os fuldføre den
sidste replik i
ensomhed


April 2007
                  "),
("Månelys",2,"Upartisk

Kun  det
stille vand 
spejler



November 2006"),
("Månelys",558481276928,"Vinterregn

Stille regn i
en fortætning af
det grå der ikke
er farve men en
sart omfavnelse af 
landet som det
ligger stivnet i  
troen på nyt liv 
indkapslet under  
nødvendighedens maske



Januar 2007"),
("Månelys",283602354180,"Vintertræ

blodårer på
himlen 
en ildrød flænge
bagved
som et løfte om
et kommende
forårs kraft
arrede grenes
afmagt



December 2006"),
("Månelys",404619392,"Vinterøje

Nærmere end dette
kommer jeg ikke
den bidende vind
frostblå af trods mod
mørkets vækst
Nær jorden en
flagrende fugl
betroet
blodets mærke



Januar 2009"),
("Månelys",558565457936,"Violer

Tænk at nogen kan
skabe violer af den
sorte muld
Og lærken der
hænger på
himlen
Selv du, min ven er
vel udlånt af jorden
men hvorfra kommer
troen og håbet og
kærligheden
Jeg bøjer mig og
plukker en blomst der 
dufter af
evighed


April 2008
                  "),
("Månelys",9848324128,"Vores

Vi plukker blomster vi
ikke skabte 
Vi opmåler jord  og
kalder den vores
Vi kalder hinanden min
familie,  min slægt, mine venner
naboer
landsmænd
tilhængere 
modstandere
fjender 
Vi elsker og hader
Vi kæmper for det der
er vores

Vi bærer livet frem
mod livet  der 
giver og
tager 

Vi dør og bliver til
navne der lever en tid
mens andre deler det 
der var vores



Januar 2008"),
("Månelys",25803399680,"Vægt

Hvad vejer et
menneskes liv?

Hver morgen løftede
han vægten af sit liv
ind i dagen
sorteret i det skete og
det endnu ikke indtrufne og 
bar det frem mod 
natten

Og hver dag
passerede han de
levende øjeblikke der
i en ubrudt kæde stod
klar til at
bære deres del men
aldrig fik lov



Juli 2007"),
("Månelys",262146,"Årsskiftet

Dét
éne sekund
fatter vi
sekundets betydning
men ser ikke at
det gentages i
det næste



Januar 2007"),
("Månelys",1073745410,"Øredøvende

Løgnen kan ikke
råbe så højt at den
udsletter sandheden
men som det dog
runger



Juli 2007"),
("Natur",0,"Aften


Grå himmel
bag sovende træer.
Gabende huller
i bleggrå skyer.
Raslen af lænker
fra stalden.
Vinduet
drejer lidt
på sit hængsel.
En sagte vind
hvori natten hvisker
vifter helt herind.
Frugtbarhedsduft
og jublende toner
fra en aftenfugl
følger med.
En bil 
kører larmende forbi
på vejen
og et øjeblik er det
som benzinosen
kommer ind.
Langt borte
kører et tog
med en lyd
som en tung kæde
der slæbes over træ.
Stilhed igen.
Himlens lys
slukkes umærkeligt
og natten
kommer ud fra skovene
med en sagte susen.
Fuglenes stemmer
er borte
afløst af
rummets susen
og nattens fred.


                  "),
("Natur",524288,"Bombemaskiner


Med blåligt skær
på de dirrende vinger
roder de sorte fugle
mellem spaltede planer. 
Deres lange næb
fisker itubrudte
godbidder
fra åbne spalter.

Støvet
der hvirvles op
står slørfint udspændt
i luften.

Med tunge bevægelser
flyver de op
som tungtlastede bombemaskiner.
De tætte kroppe
i svanger kontrast
mod solnedgangen.

Langsomt flimrende
daler støvet
og sletter
deres spor.



                  "),
("Natur",283468103680,"Efterår


Endnu engang
bliver bladene gule
og ryger af træerne.
Hænger i et par uger
og dingler ubeslutsomt
på stilkene
inden de endelig
giver slip og lader sig
falde ned på den
novemberslimede jord.
Så skal man passe på
i trafikken
et par uger.
Vådt og fedtet føre.
Det er egentlig lidt
besværligt med al den
natur der pludselig
bliver sjusket.
De må kunne gøre
noget ved det.


                  "),
("Natur",4456448,"Efteråret


Efteråret ligger gemt 
i levende hegn 
der gennemkrydser 
afsvedne marker. 
Nyvendte agre 
med nøgen muld. 
Fuglenes vinger 
synger om vinter 
medens de rastløse 
farer forbi 
de åbnede buske 
med høsten 
i favn. 
                  "),
("Natur",1081344,"En forårsdag

Jeg lægger mig i
læ af diget og
nyder lyden af
den skarpe vind der
glemmer mig og
passer sit mens
lærken synger som
kun lærker kan en
forårsdag i
solskin
Der er kun
det og ellers
intet uden himlens
blå der sluger
mig og danser med
mit hjerte


                  "),
("Natur",275012124672,"Grantræet


I den gamle mands have
står et grantræ.
Jeg har set på det
i mange dage.
Om aftenen når
himmellyset er tændt
stirrer det knejsende mod et
punkt i universet jeg
ikke kan finde.
De strittende grene aftegnes
underligt skarpt 
med hårde strøg.
Stormene har i tidernes løb
bøjet stammen så jeg
til morgen.
I mørket ses det ikke.
Måske retter træet sig
under sin spejden
mod centrum.



                  "),
("Natur",262144,"Oktober

Over brune marker 
sejler skyggernes musik. 
Efterårsklange 
farer gennem hegnene
og anslår 
dristig 
kolorit. 
Sære tegn står 
malet med vintersnare vinger.


                  "),
("Natur",274877906944,"Popler


Popler
der duver
mod mørkeblå himmel.
En jetter der
trækker sit spor.
Susende står de
med vuggende grene.
Maskinens tordnen
slår mod stammerne
der bøjer sig under presset.
Et øjeblik senere
svinder larmen
men himlen 
er stribet endnu.

                  "),
("Natur",262144,"Rudesnak

Jeg står ved mit vindue
og digter med sneen derude.
Vildt fygende farer den
forbi
og lejrer sig
mellem havens buske.

Nu og da får en
hvirvlende vind
den til at danse
så jeg med et kigger
gennem et mangedoblet slør.

Måske burde jeg gå ud
i sneen
og lade mig blæse gennem
naboens hæk
for at falde til ro
i et tulipanbed
omgivet af millioner
af sovende snesøskende.

Jeg bliver inde
for sneen er kold
og smelter mod min rude.



                  "),
("Natur",524288,"Sjagger


Sorte grene.
Æbler som kloder.
Sjaggerfløjt
fra stribede
struber.

Fløj de herhen
en tidlig morgen
mens solen
stod
rød og rund
som kæmpefrugt
over skoven
med nattens dis 
som gammelt guld?

Fløj de herhen
over solens
skive
som travle
prikker i 
sort kontrast?

Var de her tavse
da lyset
tog farve
af dagen
i øst?


                  "),
("Natur",524288,"Solsort


Skrig solsort - skrig
for jeg kan ikke
skrige
som du
mod den luende sol
når den synker.

Skrig solsort - skrig
i den dødsstille 
skov
så de vældige træers
knyttede stammer
splittes og åbnes
for lyset
der kæmper.

Skrig solsort - skrig
så det gjalder
som lurspil
fra verdens tag.

                  "),
("Natur",524288,"Sorte fugle


Morgengyldne solflader.
Sovende huse.
Sorte fugle
flakser
på baskende vinger
mod skoven.
Natten flygter.
Lyset og dagen vokser.
Mellem stammerne
lurer mørket.
Mod aften
kravler
skyggerne
ud derfra.
Lister til husene.
Klæber sig fast.
Sorte fugle
lander på taget.
Fra deres næb
damper natten.
Mulmet gror
mens
fuglene
våger.


                  "),
("Natur",65536,"Stilhed


I nattens stilhed
åbner jeg
vinduet
i taget
op i
stjernehimlen.

En ugle på
antennen
flyver væk
på tyste
vinger.
Den skriger
borte i 
plantagen.

Træerne derovre
stikker grene op af tågen
der svæver
over jorden.

Jeg lytter
efter natten
men hører
bare dryppet
fra træet
bag skellet.

Et sted i
huset
ligger mine
unger
og sover
sære lyde.
De pruster trygt
og vender sig
mens jeg er lykkelig
på kolde, nøgne
fødder.

                  "),
("Natur",0,"Trætoppene


Ovre i trætoppene
hvisker vinden.
Ovre i trætoppene
leger min drøm
mens solsorten
rasler i løvet
og natten
føres forbi.

Ovre i trætoppene
synger vinden.
Ovre i trætoppene
blinker en stjerne
og skyerne
drager forbi
i det fjerne
bag markernes
sortbrune hegn.

                  "),
("Natur",0,"Udsigt fra tagvinduet


November i
stribede felter
af brunt og
grønt.
En traktor
med
brummende motor.
En fugleflok
oppe i
hvælvingens
skær,
en musvåge
længere nede.
Deroppe
hernede
et
gyldentbrunt
skær
af frugtbarhed.
Ventende grøde.


                  "),
("Natur",262144,"Vinter

Korte og mørke dage
grågusten sne
under nøgne hegn.

Markernes sjuskede flade
åbnes og vendes
til vintersort muld.

Agerhøns svirrer i flokke
lavt over kålmarkens
grågrønne knolde.



                  "),
("Opdagelsesrejse",1030,"Aldersprivilegium

Med tiden bliver der
flere spørgsmål
end svar - og 
færre der 
spørger om 
ens mening


Oktober 2006


                  "),
("Opdagelsesrejse",16785924,"Ankomst

Jeg er gået til 
verdens ende og 
nu er jeg  kommet 
frem til mig selv hvor 
jeg hele tiden
har været



August 2006"),
("Opdagelsesrejse",34304,"Anmeldelse

Det værste er at
det ikke
må
forestille noget,
ikke bære en
kommunikation
man kan
forholde sig til
Det værste er at
det er så
ligegyldigt at
det end ikke
gider spille bold med
ens fantasi eller
bare lirke lidt ved en
anelse
Det værste er
at man er nødt til at
blive udenfor som
en der ikke er
inviteret med ind
Det
værste
er 
at 
man bliver i tvivl om
det er en selv der ikke
har fattet en skid



August 2006"),
("Opdagelsesrejse",68786620418,"Besindelse

Der er så
lidt kærlighed,
så meget had
Så lad da
kærligheden
være uanset
dens form



Oktober 2006"),
("Opdagelsesrejse",695819346464,"Beton

Sandheden ligger
bag den inderste
hinde 
Angstens betonmur
holder endnu men
vi ved begge at vi
leger som børn
der kaster lange
skygger på den
nøgne flade
Se, jeg fanger en
sæbeboble på min
hånd, et
regnbuefrø¸ fra
himlen


Oktober 2006"),
("Opdagelsesrejse",558346141732,"Bladenes dans

Himlen samler lys hvor 
vinden plukker blade og
hun går på solens
stråler gennem
gyldne sale og
ved, at hun skal intet 
vide og intet
bringe med  kun 
følge den store 
bevægelse hun ser 
gennem bladenes 
dans 



November 2006"),
("Opdagelsesrejse",25821224980,"Blinde vinkler

Sommerens blomster var
visne længe inden hun
så dem og livet
næsten forbi før hun
åbnede døren til
det hun havde set som
drømme i andres blik
Sommerens blomster
var visne da hun
bøjede sig og
plukkede en buket af
de blinde vinklers
høst
Ingen fængsler sin
egne skygge men
så længe kærligheden
vokser er der
håb endnu


August 2006"),
("Opdagelsesrejse",8809119760,"Blomstring

Det tager et
øjeblik at
blomstre
Under himlen springer
livet ud i en
lovsang mens
den øde jord
giver alt den
gemmer og i
dine øjne ser jeg
det der er 
større end os
Livet vil det og
vi vil det fordi 
vi er 


August 2006"),
("Opdagelsesrejse",2228224,"Blå

Havet
Himlen over
Klinten, en
gul kommentar


September 2006"),
("Opdagelsesrejse",0,"Brev til

Skyggerne på
muren,
en flygtig håndskrift der
forsøger at fastholde
nuet på evigheden


Juli 2006"),
("Opdagelsesrejse",8591253632,"Brikker

Ingen dag er 
uden betydning
Som perler på en 
snor holder livet vore 
dage sammen og
forbinder dem i det
livsløb vi tegner 
på tiden
Ingen dag kan
fjernes fra sin helhed
så lidt som en brik fra
et puslespil, for 
hver dag er skabt
til den plads der
kun kan fyldes
af  den



Oktober 2006"),
("Opdagelsesrejse",8606711812,"Broer

Broen står på
menneskets magt
Skyerne følger
vinden og 
kaster skygger på
de skarpe flader
Drømmene har
fundet hjem igen
til en stille krog
i mit hjerte


Oktober 2006"),
("Opdagelsesrejse",8590204928,"Cyklus

Forårets voldsomme
vækst afløses af
modenhedens rolige
eftertænksomhed
Endnu i bitre vinterdage
hæver gamle vækster
sig over jorden
som et varsel om 
det kommende



August 2006"),
("Opdagelsesrejse",12884935168,"Den lille Ole

Kun månen og
jeg er oppe
Et enkelt dryp fra
et træ rammer
jorden som en
stopklods
Det er tid at
gå hjem for  en
artig dreng




August 2006"),
("Opdagelsesrejse",10872848512,"Denne dag

Ikke i dag
men i morgen eller 
en anden dag der 
ikke ved hvad der
kommer
Denne  dag skal
strømme frit og ikke
tømmes af mine
hænder der altid
har et andet budskab
end den nyfødte 
morgen
Himlen løfter sig
højt over byen og
lader sig fylde af
øjeblikkets farver




November 2006"),
("Opdagelsesrejse",558479966240,"Det evige liv

Og uden ophør
strømmer livet frem
mod  sit udspring i
den form der er
tilpasset øjeblikkets
mulighed
Intet standser livet der
kun kender døden som
en ændring af
vilkår




November 2006"),
("Opdagelsesrejse",8606883856,"Dild

Dilden hævder sine
kroner over den
mørke jord 
Vinden modellerer et
ansigt et 
menneskestid

Himlen svøber sig
tæt i blå


Juli 2006"),
("Opdagelsesrejse",16810496,"Dyd

Jeg er den
levende dyd
i dag
Godt gået, du gamle!

Men det havde sgu 
været sjovt


Juli 2006
                  "),
("Opdagelsesrejse",549906817024,"Floden

Flodlejet eller
strømmen selv i  
sit leje på vej mod
havet
Det belivede, skabt
til at miste eller
livet selv der
trodsigt finder 
vej til sin
bestemmelse


Juli 2006"),
("Opdagelsesrejse",8640389124,"Form

Et vindpust og en
duft som ikke er
der, en tanke
der er glemt før den
dukker op men
stærk nok til at
åbne det der
heller ikke er 
som andet end et
spor der troligt er
fulgt som
var det  eneste
adgang til nuets
erindring om sol og
sammenhæng,
en ramme om den
givne mulighed


September 2006
                  "),
("Opdagelsesrejse",25954386432,"Forvist

Han ser det før
sine øjne, 
hører det før
sine ører, vil 
intet vide men
gemmer alt i det 
smertens tempel han
bygger i sit indre
Moloks ild æder
sine ofre bag det
hårde ydre som
ingen tør 
komme nær



Oktober 2006"),
("Opdagelsesrejse",10066372608,"Fred

Hvad vil vi med
fred når uretten
hersker
Så længe mennesker
trædes under fode
er fred en hån
mod livet


Oktober 2006"),
("Opdagelsesrejse",94539721216,"Fremmed

Han er der og
han er der ikke
Stjernerne er der
ovenover
og han holder stadig
fast i sig selv
som for ikke at
forsvinde
balancerende på en
spinkel tanke som en
tone af sølv der
forbinder ham til
sig selv
Han er der og
er der ikke
for ingen ved af den
der ikke vil ses
og ikke har brug for
andres lys
Han er der og
han er der ikke
Stjernerne er der
ovenover og han
går som for sig selv og
ved hvem han er
det sted han ikke
kan hører til


August 2006"),
("Opdagelsesrejse",8606752768,"Fri

Lad navne rive sig
løs,
kroppene ved
bedre
og nogle veje
kan først betrædes
når man opgiver
sig selv




Juli 2006"),
("Opdagelsesrejse",1342188032,"Genskær

Skyggerne maler på
væggene,
hastige strøg over det
hvide tapet der
intet ved og intet
husker som var
alt  tilladt og
glemt inden
næste strøgs udspil
De nøgne flader
lader sig bruge af
bevægelsen udenfor 
der kun vil sit eget 
og ikke vedkender sig 
sin egen skrift

Det står i lyset



Oktober 2006"),
("Opdagelsesrejse",8589934592,"Grænser

Gadelygter der
spidder mørket og 
afslører flisernes
hensigt
Et vissent blad og 
den smule vind der 
skal til 
En lille hund med
en i snor
Den der træder på 
streger skal dø


Oktober 2006"),
("Opdagelsesrejse",9814712832,"Handling

Drømmene,
altid drømmene først
og handlingen som
et koncentrat eller 
mindre end det, en 
antydning, et 
kejtet forsøg som en 
anelse om noget der 
kunne have været
Så meget visner
under nuets spotlight
af mangel på den
brændende tro der
bæres af modet og
styrken til at
fravriste livet dets
gaver



August 2006"),
("Opdagelsesrejse",4296015872,"Healing

Natten er næppe
foldet ud men
allerede lægger
duggen en kåbe
over den brændte
jord som en
lægende hånd på en
feberhed pande
Så meget vokser i
mørket og den
nye dag vil
finde alt beredt



August 2006"),
("Opdagelsesrejse",8657076240,"Hjertefri

Jeg lagde bare to
røde roser og
ovenpå lagde jeg mit
hjerte inden jeg
gik min vej
Jeg har ikke savnet det
siden for sådan er
det med hjerter
at har du ingen så
mærker du
slet ingen ting


Juli 2006"),
("Opdagelsesrejse",77376618496,"Hjerterum

Hjertes tomme rum
råber

Der er et
rum i mit 
hjerte der er 
dit

og du ved som af intet 

Jeg fylder det med
sol  og varme tanker og
den dag du vil derind
vil du finde
svaret på dit
kald



August 2006"),
("Opdagelsesrejse",0,"I lys

Den lyse vej
fører over havet
Landet bag har
gemt sit ansigt 
Himlen har et
mågeskrig parat


Juli 2006"),
("Opdagelsesrejse",8590753796,"Intet andet

Den røde bold
Havfladen
Himlen
Intet andet
Duggen i 
græsset
Den snert af
kulde der
trækker stæreflokke
og kaldende gæs
Den røde bold i
havet der
binder øjet 
Lyden af tiden
fra himlen og
skridtene fremad i
det våde græs



Oktober 2006"),
("Opdagelsesrejse",16,"Isop

Isopen flammer sine
blå fakler
Himlen tænder som
af sig selv


Juli 2006"),
("Opdagelsesrejse",78383187456,"Jaggedadajaggedada

...og så sagde hun hvad
jeg sagde men hun
sagde ikke det hun
selv sagde først for så
havde hun næppe
høstet den beregnede
støtte der blev rakt hende
bordet rundt...


Juli 2006"),
("Opdagelsesrejse",8589934592,"Kritik

Der er for meget 
åbe de båbe de bej
og for lidt huk hak smak
og jo et totalt fravær af
hopsa hopsa hej


August 2006"),
("Opdagelsesrejse",512,"Krænkelse

Den, der krænker et
barn, slår hul på et
univers,
som den der 
bryder seglet om et 
voksende foster



November 2006"),
("Opdagelsesrejse",627065282564,"Kun

Husker du vejen
vi gik
De lange øde stræk og
broerne vi brændte
De mange vi 
gik forbi og
kampene med dem vi
glemte var 
undervejs som
vi selv
Husker du 
fortvivlelsen og
det mål af 
glæde vi 
aldrig turde 
tømme helt af 
frygt for ikke at 
have nok
Husker du mon stadig det 
vi aldrig tør 
dele med 
hinanden fordi vi
glemte håbet og 
troen og 
os selv
Tror du mon stadig at 
nogen ser os og 
venter forude eller 
går du kun med det der
skete, et blad der 
flyver for vinden,
et strejf af sol over
dit hår

September 2006
                  "),
("Opdagelsesrejse",1090521088,"Ligestilling

Ligestilling har intet at
gøre med den
enkeltes bidrag til
samfundet
Ligestilling handler om
muligheder og 
menneskeværd


Juli 2006"),
("Opdagelsesrejse",0,"Lykken

Det er de færreste der kan
plukke lykken på 
træerne men
med lidt held kan den
sås  og ved omhyggelig 
pasning bringes til 
at spire og vokse op



August 2006"),
("Opdagelsesrejse",67141634,"Længsel

En kærlighed der
aldrig får udtryk
ligner smerte


Juli 2006"),
("Opdagelsesrejse",10242,"Mentor

Turen op ad
bakken koster
mere end
tårer og blod
Vejen ned er på
første klasse


August 2006"),
("Opdagelsesrejse",68719478786,"Moderation

Sandheden frem for alt -
men somme tider skal man
lukke sine øjne

Juli 2006"),
("Opdagelsesrejse",25820168704,"Modløs

Et smil spændt ud
mellem  god
vilje og bange
anelse 
Et blik der
sigter lidt ved
siden af 
En flugtvej 
bygget af 
alle ressourcer



August 2006"),
("Opdagelsesrejse",8640831488,"Mågeskrig

Mågerne skriger
bag himlen

hvorfra kender de
denne længsel der
ikke lader sig se

drømmen bag 
drømmen

himlens blå 
uendelighed






Juli 2006"),
("Opdagelsesrejse",8590065664,"Måner

Der findes store
måner og små i 
forskellig farver
Der balancerer en på
genboens tag der
er dyppet i orange
Nogen har skrevet et
budskab i sølv der
ikke kan tydes


August 2006

                  "),
("Opdagelsesrejse",4563402752,"Månesats

Gaderne ligger under
himlen der opfører sin 
symfoni - nu  med 
måne  bag 
disciplinerede skyer
Skygger på
revnede fliser mellem 
gadelygter der
kaster dissonanser
fra lydefri rækker



November 2006"),
("Opdagelsesrejse",12884901888,"Månestrå

Månestråle mellem
våde strå , en 
bortkommen tones
jagt på  harmoni
Natsværmere lodner
grøftens mørke
Kælighedens skarpe snit


August 2006"),
("Opdagelsesrejse",78387383808,"Naboen

så sagde han og
så sagde jeg og
så sagde han og så
sagde jeg ligegodt
og så havde han
ikke mere han skulle
have sagt
nej


August 2006"),
("Opdagelsesrejse",8607834379,"Nyfødt

Et nyfødt barn er
som et uudforsket
kontinent,
en stjerne sprunget 
i blomst på
himlen


August 2006
                  "),
("Opdagelsesrejse",16818688,"Nødråb

Jeg har to tunger
i dag
Kan nogen
styre dem begge 
Jeg har syet
munden sammen og
håber inderligt på
hjælp


Juli 2006"),
("Opdagelsesrejse",16820738,"Omstændigheder

Jeg kunne have
gjort det så
det er en dom over 
mig selv 
hver gang jeg
dømmer en anden



November 2006"),
("Opdagelsesrejse",25920798720,"Opdagelsesrejse

Med solen i den
ene hånd og
månen i den
anden
balancerede hun
over stjernerne
for at nå frem til
den indsigt hun
ikke havde opdaget  i
sit hjerte,
det sted  hun altid 
var gået forbi



Juli 2006"),
("Opdagelsesrejse",1208004096,"Opportunist

Så såre, så tit,
så ofte, bare 
engang imellem,
når lejlighed byder
sig,
så er det til at
holde ud 
Ingen kender jo
dagen før
solen går ned og
så er der hele
natten


August 2006
                  "),
("Opdagelsesrejse",0,"Orden

Hunden ligger
under stolen
Vi sover
på hver sit
niveau


August 2006"),
("Opdagelsesrejse",0,"Oversigt

Den første regn
kommaterer jordens 
tørke
Ingen udsigt til
punktum


Juli 2006"),
("Opdagelsesrejse",34304,"Prikken

Han er som en 
æske knappenåle
nogen har rystet
til låget er
faldet af -
og nu kan ingen
få det på 


Oktober 2006
                  "),
("Opdagelsesrejse",1124073472,"Privat

Ved et tilfælde
fandt jeg et
stykke af
evigheden
Muligvis kan jeg
finde lidt mere,
bare et lille stykke der
passer  med det første
og hvem ved, måske
kan jeg en dag
slå mig til ro i
min egen 
uendelighed



Juli 2006"),
("Opdagelsesrejse",549890031616,"Regndråber

Regndråberne 
hænger på
torne 
Sitrende glorier
over jordens tørst
Lyse smæld
forkynder himlens
offer



Juli 2006"),
("Opdagelsesrejse",69793226754,"Ro

Jeg har kun den
stilhed
du giver mig

November 2006"),
("Opdagelsesrejse",67142656,"Romantik

De flettede fingre i
måneskin og
vinden hørte det
hele og blæste det
op i orkan
I dag er
vinden flovet af og
ikke en finger er
at finde


Juli 2006"),
("Opdagelsesrejse",565760,"Sang

Jeg er en
solsort der
synger fra
taget
eller har jeg kun en 
solsorts hjerte
altid på vagt, parat 
til flugt
Den glødende himmel
vil dagen og livet
og  jeg tør end ikke
tage det der 
er mit


Juli 2006"),
("Opdagelsesrejse",50331648,"Sansevold

Vi er låst inde
bag vore sanser
og den der
havde nøglen har
smidt den
væk så det er
op til os selv at
finde vejen ud 
gennem hjertet og
forstanden

August 2006
                  "),
("Opdagelsesrejse",16809984,"Skyld

Hvad gør snefnugget her 
så langt fra vinter,
sommerfuglen bag den
blinde rude på det 
støvede loft
Er  edderkoppen i
nettet byttet


September 2006"),
("Opdagelsesrejse",69860360192,"Skørt

Man kan knuse
et hjerte
med ét blik



August 2006"),
("Opdagelsesrejse",8591548416,"Solsang

Solsorten synger 
Flasken er ikke
tom endnu og
om lidt tænder nogen
stjernen over hylden
der altid gør mig
svimmel


Juni 2006"),
("Opdagelsesrejse",77309445632,"Spontanitet

Helt af sig selv
siger munden de
ord du gerne vil
høre og nu de er 
ude kan vi jo
lige så godt
benytte os af dem
men når jeg
går tager jeg dem
med og jeg 
kommer ikke igen


Juli 2006"),
("Opdagelsesrejse",8589968896,"Storsind

Det fylder mit
hjerte og jeg
rækker hænderne
frem som en 
skål fyldt med
den bedste hensigt,
min tilgivelse, 
mit storsind

Holdt fingre tæt
måtte sand tørste



November 2006"),
("Opdagelsesrejse",558346313856,"Sølv

Hvad vil himlen
med den fugleflok der
forsvinder gennem
sorte huller,
kaldets ekko fra 
mit hjerte
Sølv er dagen 
over engen



                  "),
("Opdagelsesrejse",69793253888,"Tandløs

Jeg vil ikke men
kan ikke se
vejen uden om
Tænderne er
spyttet ud for
længe siden så
hvordan bide det
i sig
At sluge råt er
ubærligt




September 2006"),
("Opdagelsesrejse",549755813890,"Test

Hvis din gud
inspirerer dig til
at fordømme
andre er det ikke
Gud du har
mødt men et
spejlbillede af
din egen synd


August 2006"),
("Opdagelsesrejse",0,"Tid

Så længe du vil,
så længe lyset vil og
bagefter tager duften
over og fortæller
for mørket


Juli 2006"),
("Opdagelsesrejse",8724160512,"Tidehverv

Og i den halve skygge står
en spæd forundring og
rækker efter sig selv
som et forår der
endnu ikke tør
kende sit værd
Dagen er gammel men
endnu har solen styrke



Juli 2006"),
("Opdagelsesrejse",618877952000,"Tilgivelse

Stjernerne husker og
har skrevet deres
tegn på mørket
Intet glemmes i
det store øde men
over dybet
svæver stadig
begyndelsens vilje
til liv



September 2006"),
("Opdagelsesrejse",558345789444,"Tomhedens tyngde

Kurven vokser sig
tung i de 
lange skygger
Så meget gik tabt i
det store spil, 
spildt, overset, forkastet,
Lad der være et
hjerte bag
mørket,
et hjerte større end
mit



Oktober 2006"),
("Opdagelsesrejse",17301520,"Trio

Den fugl kan se
hvem jeg er
Rosen svajer på
grenen og kan
ikke flyve væk


Juni 2006"),
("Opdagelsesrejse",824767948800,"Træet

Pluk mine blade,
brænd mine kviste
Over den sorte jord
hæver jeg klodens
insisteren på liv
Pluk mine frugter,
hug min stamme
omkuld,
livet vokser uhindret
af  uforstandiges
hænder
Den nye verden er
allerede født og
bæres ind i lyset 
af det liv der
vil livet



August 2006"),
("Opdagelsesrejse",8591015936,"Udveksling

Stenen fra
stranden ligger
stadig i lommen
Til tider
tager jeg den i
hånden og 
tager ved 
lære
Jeg betaler med 
indre varme 
Vi trives


August 2006"),
("Opdagelsesrejse",8589976064,"Uegentlig

Træet er ikke
træet og 
skyggen er ikke
skyggen
Dagen er ikke dagen
men en anden dag
han ikke vil
åbne fordi den 
skjuler det
kommende, det
fuldbyrdede,  knoppernes
udfoldede pragt
Han ser en sommerfugl
flyve men
læser ikke dens
flugt



Juli 2006"),
("Opdagelsesrejse",25937879044,"Vintring

Kraften
driver foråret frem i
kaskader af liv
og han ved sig selv  at
for ham
går det stadig
mod vinter
Han husker men
husker intet, kun
længslen,
glæden og smerten
Et fuglefløjt i den
lyse nat da han
gik med én 
ved sin side



August 2006"),
("Opdagelsesrejse",8590458880,"Visdom

Skyggeleg på
muren,
en flygtig skrift
der repeterer et
budskab
Solsorteunger
staver i kor med
stemmer hæse
af ægget


Juli 2006"),
("Overvejelser",1107306496,"Ørkenvandring

Uden mure,
uden begrænsning, som en
ansigtsløs ørken

Uden mening 
Uden værdi

I grå kolonner
plukker de
løs af det efterladte

fylder sig over et
bristepunkt
som  forsvar mod
tomhedens vækst

Korsfæstede til
begæret forbander de
hinandens blodige hænder

Kun græsset lytter til
vindens sang



August 2005
                  "),
("Overvejelser",16777218,"Afstand

Jo længere du
søger indad
jo længere 
kommer du
væk fra
dig selv


September 2005"),
("Overvejelser",8192,"Anonym

Som passagerer på en
banegård
haster tankerne igennem
for at forsvinde 
gud ved hvorhen med
deres personlige
bagage
Nogle efterlader
tydelige spor der er
besværlige at fjerne
og enkelte bliver hængende
så længe at der  må
tages affære

En del har aldrig købt
billet



Oktober 2005
                  "),
("Overvejelser",8192,"Approberet

Vi står i kø foran
vinduet for at
betragte udsigten
Bag gardinerne anes
den blinde vinkels
overgang til
uendeligheden
Et sidste penselstrøgs
beroligende kærtegn


August 2005"),
("Overvejelser",2,"Balance

Nuet 
balancerer på
erindringen
mens det
pejler sig ind på
fremtiden


August 2005"),
("Overvejelser",17179869184,"Bare

Det er jo ikke
fordi men
alligevel
jeg mente bare..
men det er nok
bare mig...
og det er jo
helt ligegyldigt



August 2005"),
("Overvejelser",0,"Brand

Den flammende morgen
angriber himlen og
kaster efterårsløvet ind som
sit  sidste argument

Dagen tørrer tavlen
ren og  lader sig
blæse tom for
alt der har skærmet
tilvækstens  tankespind

For alle at se
tegner livet sig
ind til sin prøve



Oktober 2005
                  "),
("Overvejelser",16777216,"Brikker

Hun lægger først de
pæne,
så de grå der bare
fylder 
Til sidst den 
lille håndfuld hun
aldrig kigger på men
får på plads

Herregud, hvor mange  
skal der til for at
skabe et
menneske


Oktober 2005
                  "),
("Overvejelser",32772,"Dengang

Gaden husker og  sender 
billeder fra en lys dag
Der skal sigtes præcist for at
ramme smerten der
gemmer sig bag en
øm vemod

Og jeg har intet at
sende tilbage


Oktober 2005"),
("Overvejelser",618542399520,"Destination

Med ét er
rejsen slut og
vi ser at målet
ikke er her men
tiden vi 
tilbragte sammen



Oktober 2005"),
("Overvejelser",549755813888,"Det

Det står ikke i
bøger skønt der
er skrevet bøger nok
Det gemmer sig ikke i
en organisation eller
bag en vedtaget verdensorden
Måske er det snarere
lærredet du er
malet på,
instrumentet der
spiller din væren

kilden hvor du
henter din
længsel



September 2005"),
("Overvejelser",68719478786,"Diplomati

Selvom du 
vil stå på hovedet
kan det jo godt være
jeg hellere vil stå på
mine ben -
og hvad så? -
Vi taler vel bare
lidt højere



August 2005"),
("Overvejelser",0,"Efterladt

Da han så op var
tiden vokset fra
ham og alle
gået
Han kaldte længe men
kun ekkoet kom 
kastet tilbage fra 
sprogets grænse

Fastfrosset under
tiden i vent bag
mælkede ruder


August 2005
          "),
("Overvejelser",16777222,"Erfaring

Man bliver ikke
klogere med alderen men
påtvinges et andet
perspektiv


August 2005
                  "),
("Overvejelser",69793218560,"Fælles front

Vi mener
eller gør vi 
det
Jeg mener i
hvert fald eller
gør jeg
Jeg mener det 
samme som dig,
det er det jeg
mener



September 2005"),
("Overvejelser",8192,"Facit

Vi tror vi har
fundet sandheden
men glemmer at
konferere med vore
drømme
Sandhed lader sig
fortolke og end ikke
historien formår at
skelne mellem de to



August 2005"),
("Overvejelser",12884902400,"Fastklemt

Natten falder på mig og
falder hårdt
fastklemt til mørket
må jeg se mine
tanker i øjnene men
alle slår blikket ned
eller kigger væk



September 2005"),
("Overvejelser",134217744,"Fnok

Ræk hånden ud og
lad muligheden lande -
pust, og sæt en klode i
drift

Svævende sigter livet
på sin chance



September 2005"),
("Overvejelser",69860360192,"Fortroligt

Jeg ved det
Du ved det
Vi ved det
ikke



Oktober 2005"),
("Overvejelser",68719478784,"Frieri

Jeg  -
forekommer for
snævert
Man  -
rammer for
bredt
Tør jeg foreslå
vi?



Oktober 2005


                  "),
("Overvejelser",2048,"Frugt

Freden hænger som
en frugt mellem
andre frugter og
venter på
plukkerens sult



September 2005"),
("Overvejelser",68786585602,"Gartner

Kærlighed er altid
en ufortjent gave
men det nytter at
leve som om det er 
noget man  dyrker


August 2005"),
("Overvejelser",77309476864,"Glemt

slangebøsse
skabslåge
sodavaskevand

slangebøsse
skabslåge
sodavaskevand
og den køkkenblå
bukket i facon til et
køkken med en
dreng der har
hentet sine farver
fra solen og en 
kone der
vasker glas uden at
se på en dreng der
intet fortæller for
hvorfor skulle han
det og hun siger
nåe, er du sulten for 
det er han jo nok og
han nikker og hun 
tørrer sin hænder i
det ternede forklæde og 
han sætter sig ved siden af 
komfuret med det 
lyse hår i 
øjnene og
trommer gummisko på 
høkassens sider som for at 
telegrafere en 
besked til en der 
forstår og
måske går den igennem for 
hun stikker ham en 
rundtenom og siger 
stik så af og
han er væk som en 
skygge inden hun har fået
hænderne i vandet hvor hun 
genfinder tanken på
kanten af et smil 



September 2005"),
("Overvejelser",16785408,"Grundvæv

Jeg bor i det
land jeg vævede af
mine drømme
længe før nu
De enkelte detaljer
stiller mig til
regnskab og kræver
forklaring på
trådenes sammenhæng og
forløb i den
helhed jeg
endnu ikke har
kunnet beskrive 
fordi jeg aldrig har 
magtet bevægelsen ud over
rammen



September 2005"),
("Overvejelser",78483816448,"Hengiven

Du vil have
en beundrer
og jeg bliver en
beundrer
Du vil have en
slave og jeg bliver
en slave
Du vil have en
hengiven ven og
jeg bliver en
hengiven ven
Jeg får dig men du
får ikke mig



September 2005"),
("Overvejelser",274878955520,"Himmelsk rød

Modne rønnebær
dupper med rund overflod
et digt på himlen




September 2005"),
("Overvejelser",0,"Hop

Min tanke hopper
som en gummibold
hop
hop
hop
den har det med at blive
standset eller
trille væk
Så må jeg lede og
sparke den i spil



August 2005"),
("Overvejelser",0,"Hopla

Jorden under
himlen over
Jorden over
himlen under
Jorden under
himlen over
Jorden over
himlen under
Jorden under
himlen over


September 2005"),
("Overvejelser",549755813920,"Høst

De smiler fra
deres grave som
dem der ved fordi der
er en erfaring til
forskel
De visne blade
falder til ro og
lægger et gyldent
kærtegn over et
ukendt navn




Oktober 2005"),
("Overvejelser",68786585600,"I spil

Midt i larmen griber jeg
dit blik og putter det i
lomme
Veloplagt nyder jeg
varmen og sender præcise
staccatoøjne tilbage



September 2005"),
("Overvejelser",68786587648,"I vejen

Der er det og
det og det 
i vejen og
ikke det der ikke er
noget i vejen med
men det der 
for alvor er i 
vejen det 
taler vi ikke om for
det er jo 
det og 
det 
lader vi ligge




September 2005"),
("Overvejelser",1073743872,"Indsats

Mine ord
indkredser dine og
lægger sig over dem som
en dyne
De lyder bedre med
kvalte kanter så  nu og da
lader jeg en passende
mængde luft 
slippe ind

Det kan vel blive til
noget engang når
branden er under
kontrol




September 2005"),
("Overvejelser",618492068864,"Kalkulation

Nogle regner i
penge og
menneskeværd-
ja, er overbevist om at
de trækker renter i
himlen



Oktober 2005"),
("Overvejelser",2147483648,"Kamp

Dagen kaster sig 
over natten og 
river den op i 
blodrøde rifter
Stilheden såres med
morgenlydes hug mod
den bløde stilhed
Med dristig pensel tones
himlen i stadig lysere
nuancer inden 
erobringens sidste 
strøg


Oktober 2005
                  "),
("Overvejelser",524288,"Kontrakt

Spurvene i hegnet
har besluttet sig og
som en udløst skæbne
dækker de jorden og
stempler en
mangetået signatur

Den vrimlende vilje
trækkes væk på et
durrende sekund

Det står i 
støvet


September 2005

                  "),
("Overvejelser",0,"Kopi

Man kan få stor
succes ved at
genopføre
andres liv
men kommer så
for sent til sin egen
premiere


August 2005
          "),
("Overvejelser",0,"Låst

Skyggen 
lever af
lyset og 
tager hvad den 
kan få
inden mørket
sletter alle 
skel

Solen 
følger sin
plan på 
afstand


Oktober 2005"),
("Overvejelser",8192,"Lænker

Har man ingen penge er
man slave af den
tomme pung
Har man mange er man 
bundet af  
formuens helbred
Er der hverken lidt eller 
meget  skal man 
sørge for at holde 
enderne sammen så 
resultatet bliver vel det 
samme



Oktober 2005"),
("Overvejelser",262148,"Løvfald

Bladene falder også
i mørket,
afstødes,
svæver, 
lydløst som solstrejf over
opadvendte ansigter,
rammer jorden for at 
følge dens lov


Oktober 2005
                  "),
("Overvejelser",17196646400,"Målrettet

som vinden der
bare er en
bevægelse mod
et sted der er
ro


September 2005"),
("Overvejelser",32896,"Mellemrum

Nogle dage bærer
ikke lykken i sig,
men må hjælpes

Selv dage kan
føle sig som et
mellemrum



September 2005

                  "),
("Overvejelser",558345756708,"Mening

Som lyset der afgrænser
mørket
sorgens indramning af
glæden,
de ubekymrede dages
leg med hele 
jordens tyngde
Og uden ophør  går vi
fremad,
frem mod et mål vi
ikke har valgt men 
må lære at tage imod som
var det meningen med
hele vores væren



August 2005
                  "),
("Overvejelser",559436277760,"Menneskeværd

Måske vil der
komme en dag
hvor vi uden frygt tør
række hinanden hånden
fordi vi har indset at
vores forskellighed
kæmper for
samme sag
Hvad er et menneske
andet end et søgende
svar på skaberens
frugtbare gåde



August 2005"),
("Overvejelser",1073743872,"Metode

Enhver kan få brug for at
rage kastanier ud af
ilden
Det kan gøres ved hjælp af
lange, tynde
bortforklaringer der 
holder fingrene på 
god afstand af
heden og er lette at 
stikke ind fra
forskellige vinkler
Benægtelser kan 
bruges men er 
ofte for 
klodsede
I mangel af bedre kan man 
forsøge at 
skyde dem ud med
små skarpe angreb men
skal være sikker på at
ramme præcist og
ikke for hårdt da man 
ellers risikerer at 
sprede branden



September 2005"),
("Overvejelser",32768,"Mod

Smerten mellem det
der er og 
det der
kunne være

Smerten mellem det
der er og
det der var

kan bruges som
drivkraft i
nuet

hvis man tør
gribe om den


August 2005
                  "),
("Overvejelser",2147483648,"Morgen

Som en flaske
som en flaske nogen
har tømt og
smidt fra sig under
en hæk

den tomme lugt af
derpå



August 2005"),
("Overvejelser",4328521728,"Opslugt

Og er der ingen 
Og er der kun

Stjernestrøede blade
forsøger at nå
inderlighedens center

Gennem grønne skakter
glider lyset ind i
mørket og lader sig
assimilere 

Stilheden rammes af
dryp fra et træ



August 2005"),
("Overvejelser",16777216,"Ord

Som en lille tap
på kloden står jeg
lige op og ned og
kigger ud i
verdensrummet mens jeg
drejer rundt med
jorden og ser på det
der er at se og 
jeg ser at jeg er
lille bitte og at
verden er uhyre stor
Der er lys i alle
stjernerne der
prikker himlen fuld af
ord  
Hvor er det sært at
være lille når man 
helst skal være stor



Oktober 2005
                  "),
("Overvejelser",17179870208,"Overløb

Hun har for mange
ord i munden og
arbejder ihærdigt på at
spytte dem ud
Lange kæder af
grå udredninger hægter sig
klægt på indkredsede
gidsler og forsøger at
finde vej gennem
ubevogtede øregange
Slipper de ind
slukker de langsomt
lyset i offerets øjne


September 2005

                  "),
("Overvejelser",16777216,"Overvejelse

Er det tanker
der tænker
mig -
eller er det
mig der
tænker tanker?


August 2005
                  "),
("Overvejelser",67108864,"Palet

Kærligheden har
mange farver
som regnbuen i
det grå

Himlens smil
gennem tårer


August 2005"),
("Overvejelser",2,"Perspektiv

Man ser meget når 
man kryber langs 
gulvbrædderne som
man aldrig så da man
gik med næsen i sky



September 2005"),
("Overvejelser",32768,"Posen

Så længe hun husker
har hun  samlet små 
stumper vrede og
forurettelse,
selv mindre flager had er 
lagt til side sammen med 
fornærmelser og 
frustrationer
Det hele er i posen der
ikke kan tage mere
end ikke den
mindste misstemning for 
slet ikke at tale om de 
splinter af aggression
hun dagligt
finder på gaden eller
modviljen der 
uvægerligt klæber sig fast under 
skoene
Hun må blive hjemme


September 2005
                  "),
("Overvejelser",69826772992,"Prioritet

alle ser den
fremstrakte hånd men
hvem rækker ud 
efter en
vedhæftet byrde
Et sted i tågen 
findes skjul i et
forbrødret smil 


Oktober 2005
                  "),
("Overvejelser",68786618368,"Pyt

Jeg vender mig og
smelter ind for
dine øjne

Jeg vender mig og
lader tiden
slå rod

i mit offer

Så pyt




September 2005"),
("Overvejelser",4328522240,"Råb

Natten råber mit
ekko op,
gjaldrende slag mod
tomme mure
Intetheden dikterer
sit budskab i det
øde mørke hvor
jeg sanser lyden af 
et hjerte

Intet gror på
sten og dog
er der liv nok til
smerte



September 2005"),
("Overvejelser",69860360192,"Rekonstruktion

Alt var svøbt i
det blik du
lod falde
Måske kan der
reddes lidt når
du er væk,
bare et par stumper til at
samle facaden


September 2005
                  "),
("Overvejelser",0,"Reportage

Kornet står tæt som
en sensationslysten
menneskemængde
Lavtgående flagermus
dukker ud af
tusmørket og
plukker deres bifald

Nogen har slukket for
lyden



August 2005"),
("Overvejelser",549772599296,"Retning

Vejen ligger åben for
fortolkning og  forhandling
men  fraviger aldrig
sit forløb
Der er rastepladser og
krybespor,
mulighed for skiftende
hastighed og et varieret
udvalg af kulisser
men retningen er
fremad og du kan ikke
stå af før du engang
kommer i mål



August 2005"),
("Overvejelser",68820172800,"Rum

Intet kan være så
tomt,
så øde,
så blottet for
mening

Hvad er der så med
mit hjerte og 
hvor er 
du?



September 2005"),
("Overvejelser",283469414400,"Rønnebær

Morgenfjer på
himlen
En blødende flænge i
det blå
Fuglen æder de rønnebær
ræven ikke kan nå og
de er ikke sure men
orange drømme
plukket af himlen der
gavmildt ryster sig for
farver over det
sommertrætte land
Vinden venter på
afstand på sin tid der
kommer som en lyd af
vinger der letter fra
et træ under et
fuldbyrdet løfte



August 2005


                  "),
("Overvejelser",524288,"Sart

Hav som
pudset blik
Himmel som
sart vemod
En skrigende
måges vidende
blik



August 2005"),
("Overvejelser",558749712384,"September

En dag som
denne
pakket ind i blåt
med en latter af
lakrøde rønnebær,
en dag som denne
i et ekko af gylden
høst tør man
plukke hele livets
mening som et
løftefuldt bær i
hegnet mod marken 
og et øjeblik stole på
lysets magt



September 2005"),
("Overvejelser",558480498688,"Skaden

Hen over tage og
ned mellem 
bekymrede huse
Et enkelt fejltrin koster
mere end 
løse tagsten kan
tænke sig til
Efter storm følger
storme og den der
kan regne gør
regnskabet op og
forstærker sine
bastioner
Spot ikke livet
under den vrede
himmel
De afrevne fjer 
danser i vinden 
uden tanke på
hullede vinger og
kommende kamp


Oktober 2005
                  "),
("Overvejelser",16777216,"Skal

End ikke et
øjeblik hviler hende
vilje, 
den tilslebne
flades magt over
den færdige
form,
sindets jerngreb om
strukturen
Stemmen er færdig og
ansigtets udtryk

Hun skal arbejde
mere med
lyset i 
sine øjne


August 2005
                  "),
("Overvejelser",1048576,"Som

Stammerne
tågen
lyset på skrå
som støbt i sølv
som et flygtigt kærtegn
som stråler graveret  i 
glas 
som var det
evigt og ikke et
øjeblik jeg får lov at
holde i min
bevidsthed for
ingen verdens ting



September 2005"),
("Overvejelser",627082010656,"Spillet

Nogen skriver stykket og
lader andre sætte det i
værk
Folk med håndelag
laver kulisser
Scenen befolkes af
mennesker der lader sig
instruere
Én passer fortæppet

Oktober 2005"),
("Overvejelser",558479974416,"Steppeblomster

Den lysende himmels
låg over jorden
Det kæmpende livs
eksplosion i
spinkel skønhed,
sårbarhedens blottede
center rakt frem
mod den blinde
vold og
livets mulighed der
skal hentes
Og farverne er
som vindens sang
gennem græsset,
som vildgæssenes lyde
over skyerne en
tidlig morgen
Mennesket drømmer om
kærlighed der ikke er
af denne verden og  ser,
men ser ikke 
hengivelsen til 
jordens  gave under den 
flygtige kuppel



September 2005"),
("Overvejelser",67108864,"Stjernestøv

Himlen slører dine
øjne
Med spidsen af
et kys
nipper  jeg et
stjerneskud



Oktober 2005"),
("Overvejelser",1073744898,"Tidsånd

Lad den falde der
ikke kan stå



September 2005"),
("Overvejelser",32768,"Timing

Snart 
bare ikke 
dette nu der triller
foran mig uden at
lade sig indtage
Bag efter virkeligheden
forsøger jeg at
fylde mig selv med
mening

løbende



August 2005"),
("Overvejelser",16777728,"To

Ondskabens ansigt
vender nakken til og
jeg mærker ikke
den er den samme som
min


August 2005"),
("Overvejelser",68786618368,"Tomat

Du skærer tomaten i
skiver men det
er mig du har 
under kniven
Du vil og
du skal og du må
og du måler mere
over ryggen end dit
blodtryk kan tåle
men fanden så -
jeg lægger mig 
blot og lader dig
tage hvad du 
vil for jeg
frygter ikke kniven og 
har smag for din
harme



August 2005"),
("Overvejelser",68786618368,"Udsalg

At kaste sig i
grams har sin
egne ødsle
sødme der let 
forplumres af
sure tanker om 
tomme hylder og 
ribbede hjerterum


September 2005"),
("Overvejelser",41943040,"Udsnit

I husene bor 
menneskene der
går ud og lader dem stå
tomme som efterladte
sneglehuse
Når de vender hjem
fylder de dem ud
med sig selv og
låser dørene bag sig
Tilfældige udsnit flygter
fra slørede ruder



August 2005"),
("Overvejelser",64,"Udvej

Døre forsøger at
skjule deres mulighed
Skridt ridser 
stilhed
Så mange rum i
dette nu der har
gemt sin udgang
bag tiden


August 2005
                  "),
("Overvejelser",549755813888,"Vækst


på vej mellem
jord og himmel

ad lysets vej

banet mellem
skabelsens 
nøgne hænder



August 2005"),
("Overvejelser",0,"Vilkår

Vejen ud er vejen 
ind 
Ensomheden trækker
lige linier der
lader sig beskrive med
de nødvendige ord,
acceptens beslutsomme
signatur


August 2005"),
("Overvejelser",134218240,"Vogt dig

Hækkene ved det og
alle havens
vækster
Huset kan det
udenad  men
selv spejlet holder
tæt når det skal
Gulvene knirker
af viden under 
hårde skridt og
advarer bøgerne om at
holder siderne tæt 
Tapetet dækker med
ryggen mod væggen
Og du 
der holder huset tæt bør 
ruste dig mod 
sprækker


August 2005"),
("Overvejelser",1026,"Vokseværk

Er man af den
slags der kan
vokse i skoene
ved at tage
stik hjem med
falske kort skal
man vogte sig for
loftet


Oktober 2005"),
("Perler på snor",17179869188,"Aftenunderholdningen


De sagde,
det var Schade, de hjalp ind
i auditoriet
den aften
efter middagen.

Schade?
Han er da død?

Det store hoved 
over den lille krops
besværede åndedræt .

De blanke lidt stirrende øjne
i en karruselforvirret
stemning.

En gammel mands kejtede svar på 
benovede spørgsmål.

Vil du glæde os med at læse lidt?

Læse?
Ja.
Hvor er det?
Hvor står det?

Uventet voldsomt slog kraften ned
og sprængte tiden.
Der 
var han,
Satan lyneme,
Danmarks lyse digter,
Jens August Schade

I sekunders evighed
svævende i
urtids skabende kraft
til en forpint hosten
kaldte tilbage til
kroppenes
virkelighed.

Der var boller med smør
til kaffen.


          "),
("Perler på snor",50331648,"Aldrig


Han går tur
med sin ensomhed
Bor sammen
med sine
reolers bøger og
lytter til
stemmer han kan
kontrollere på afstand

Han vil aldrig
blive forladt
Hans hjerte vil aldrig
bløde efter ord der
ikke kan sættes på
plads eller stoppes
med et tryk på
en knap

Et sted ligger en
brun konvolut med
et foto af en lille
dreng der kæmper med
et beordret smil der 
aldrig kommer til at
bo der

          "),
("Perler på snor",68719575040,"Besøg


Og under uret 
sidder de og venter
Venter under uret 
i udkanten af lampens
lysplet
Og uret tikker liv
og lampen lyser ind i 
gamle dage
der svæver fri
af tid
Og de siger lidt
til hinanden
men mest til dem
i lyset
der er de eneste der 
kommer på besøg
under urets æden liv
og lampens lysen
fortid


          "),
("Perler på snor",695801511972,"Bisættelse uden
gejstlig medvirken

Ved bisættelsen rejste
enken sig op
og stående foran
kisten fremsatte hun 
en kort udtalelse om 
afdøde som vi 
forstod  havde
klaret livet jævnt
tilfredsstillende
altid optaget af
hverdagens  praktiske
problemer som han
fandt behag i at 
finde løsninger på
Efter talen sang vi
endnu en munter 
sang fra de
duplikerede ark
som en ven af 
huset havde fremstillet på 
sin computer
Højt over kisten og den 
omgivende familie
gjorde Jesus krav på at
være vejen, sandheden og
livet 




                  "),
("Perler på snor",8640303620,"Brand

Bålet brænder ud men
kaster knækkede skygger på
væggen
Det forgangne og det kommende
i et nu uden tid

Kroppen holder ham fast og
tvinger ham ind 
Han ser uden øjne
hører uden ører og
hjertet smerter
under værdiernes
kæntring

Aldrig mere og  
nu

Mor, siger han og griber en hånd der 
et øjeblik lader sig holde indtil
en anden kalder




December 2004


Samhørende:
Perler på snor
Chefen
Hendes sidste udflugt
Brand
                  "),
("Perler på snor",95630197760,"Bøjlemanden


Da han kom hjem 
tog han manden af 
og hængte ham 
på bøjlen i entreen 
som altid 

En lang dags 
længsel 
efter 
den viljeløse 
salighed 

Usynlighedens 
ritual 

Lydløst formede munden 
det elskede mantra: 
Som du vil.. 
min ven


                  "),
("Perler på snor",25786585604,"Chefen

Han ruger på
bilen 
det store
hus og
hende der er
alt 
Den rastløse jagt
gennem territoriet der
ikke vil som han
men lukker sig om
budskabet om hendes
død der gentages 
monomant af
fremmede der vil at 
dette er hans mulighed
De overbærende smil når
chefen skrider ind før
den tavse gråd
De våde bleer




Samhørende:
Perler på snor
Chefen
Hendes sidste udflugt
Brand
                  "),
("Perler på snor",68719509568,"Døren

Da døren
lukkedes bag  
min  tillid
forlod jeg
et rum
min smerte ikke 
kan åbne
Måske får jeg 
mod til at 
vende mig 
bort og 
lade din 
varme nå 
mit hjerte


                  "),
("Perler på snor",17179869200,"Den sande
have

Der er vagtsomt
lyttende hjorte i
havens skygger
Soløer af rusten lykke og
en sælsom duft fra
orientens glemte
terrasser
Unge halvnøgne 
legemer i de blå
månestrejf,
elefenbensdrømme i
skjulte kig 
Under de gamle
roser sover
Tibets forjagne
vækster i forventning
om dét forår de 
skal velsigne
Det kommende udspring
Den fuldbårne vækst

Han inhalerer og
tænder en ny


December 2004
                  "),
("Perler på snor",50364480,"Den tredje vej


Der er ingen vej
udenom
Fremad
eller lade sig 
besejre
Jeg går
ind i
mig selv
og lukker døren
tæt


          "),
("Perler på snor",85899379456,"Fire år

Hans angst har
omringet territoriet og
ligger parat med
skarpe skud i
alle forklædninger
Med ét er han
fire år og sidder på
trappen til udhuset
mens  kammeraterne
lusker rundt med
revolvere på jagt efter
hans sjæl der 
for længst har givet
op og rakt 
hænderne i
vejret


                  "),
("Perler på snor",206158463012,"Forræder

Med håbet om
at du ikke ville vågne
gik jeg hen mod din seng
med mine blomster

Dine øjne åbnedes
og i sekundet
inden smerten tog over
så jeg
styrken
af din kærlighed
og afmagt

Ubærlige lidelse

Og mit sinds forræderiske flugt

Det er dig, der er syg
Det er dig, der er syg
Det er dig, der er syg

Udenfor
var myldretiden sat ind
                  "),
("Perler på snor",77326189056,"Frelsen

I opholdsstuen
har de anbragt en ovn

Røgen driver kvælende
ned ad gangen

Så I den ikke?
De brænder os levende
En efter en

Vi omfavner
din krops rædselskrampe

Og de spurgte
Lille frue
sagde de
Lille frue
venter De besøg i dag?
Neej
det gør jeg da ikke
Så tager vi Dem
efter middagspausen

Og se, hvad de har sat foran mig
Det er noget, jeg skal drikke
For rigtigt at kunne
flunse

Vores mod
støtter dit
og vi går ned ad gangen
så langt
at du vender tilbage
til det vi kalder
virkelighed

Jeg har nok taget fejl

Din lille krop
bag det store vindue
Og uden lyde:
Tak fordi
I frelste mig"),
("Perler på snor",68719509504,"Gensyn


Uforberedt
står jeg i
dine øjne og 
smelter væk
Forfaldne års
Berlinmur
står ikke for 
fald i mit
vingeskudte
skjul

          "),
("Perler på snor",137476752128,"Glasklokken


For 50 år siden
gik jeg her
som en dreng med
en angst 
vokset vild i 
min mave
Gik langs gærdet med
de solvarme sten
i en glasklokkes 
lufttomme rædsel
mens jeg råbte på
den mand der 
50 år efter
søgte langs
gærdets sten
for at trøste den 
dreng ingen andre
så i hans 
glasklokkes 
sprøde fængsel


                  "),
("Perler på snor",8607760384,"Graveren

Jeg graver dybt for
at forstå mig
selv men finder
intet på bunden
Jeg ser mig
omkring på min
aktivitet,
bøjer mig dybt
og rækker
graveren hånden 
Måske er der
tid endnu
          "),
("Perler på snor",206192017440,"Har valgt livet fra


Hvor ville jeg gerne
have holdt
din hånd
et øjeblik
inden du gik

Hvor ville jeg gerne
have lagt min hånd
som en nænsom ro
mod din pande

Hvor ville jeg gerne
have mod til at tro
der er kærlighed gemt 
til os alle



                  "),
("Perler på snor",17198747684,"Hendes sidste udflugt

Hun ligger på diget
På jorden
På diget
Ligger
Før gik hun
For længe siden
Fugle flyver
Der var et hus
Og de mange børn
Hvor er de børn?
Hun kan ikke høre dem
Hun har ikke hørt de børn længe
Længe
Ligger jo godt nok
Ligger
Tyngden af
det lange livs smerte
Kroppen borte i
mørket
og ude på vandet er der et lys
som blinker
Det blinker lyset
Og mor er der
Fylder det hele
Pusler i det store øde
og køkkenlampen skærer i øjnene

Sluk lyset mor





Samhørende:
Perler på snor
Chefen
Hendes sidste udflugt
Brand
                  "),
("Perler på snor",25803362308,"Lyde i støvet

Hans ben hvirvler 
støvet på vejen  for 
det er  sommer
Vejtræernes skygger er
de dødes rige og  han 
skal se sin første bil om
lidt hvis den kommer som 
rygterne vil og hans 
mund laver lyde han 
aldrig  har hørt mens hans 
brune ben bliver 
grå
Der er langt til byen og
længere til kanten af
livet hvor lydene
let forstås men 
mistolkes af
personalet




December 2004
                  "),
("Perler på snor",17196679680,"Mærker

Tabte fodsporene 
 
orker ikke at
skaffe nye

går sporløs omkring 

går ud og
hjem

går igennem

hugger mærker i
tiden for ikke at
blive væk


                  "),
("Perler på snor",17230200832,"Masken


Engang var den
kæk
fletningen
Nu er den der
bare 
mat over
skindjakken
der har trodset
alle årets tider lidt
for ofte
ligesom
øjnene der
bag de skrå
solbriller er
falmet til
mundens
vrængende
kynisme
Hun var altid
en hård lille
en
Nu græder
hendes
maske


          "),
("Perler på snor",17196650757,"Perler på snor 

Den lille pige græder 
for Guffemis er død
På altanen banker hun 
dyner mens hun 
myndigt styrer huset 
og føder børn
Hun tisser på 
stolen fordi hun har 
gemt bleen i 
klædeskabet og 
munden der styrer en 
stor familie løber over i 
tom monolog mens 
den lille pige stadig 
græder over 
Guffemis og 
døden og 
det der var 
engang




Samhørende:
Perler på snor
Chefen
Hendes sidste udflugt
Brand
                  "),
("Perler på snor",0,"Puslespil


Diget
og himlen
over havet

Stilheden
piller elementer
fra hinanden
og lægger 
puslespil af
gemte brikker
Mønstres muligheder
flammer op af
horisonten og 
ser sig om
efter rammens
støtte
Lange linier
søger fodslag
med farver der
ikke vil være med
men foreslår
ændrede sammenhænge
i et billede der
endnu ikke
kender sig selv

Fugle i
formation
hamrer hul
i himlen

          "),
("Perler på snor",17196646912,"Revanche



Alene
drog han
ind for at 
genvinde
det tabte land
på sit sinds
blodige slagmark


Han lever endnu

Du ser slagmarkens
kaos

                  "),
("Perler på snor",41943044,"Rude

Med et grimet
smil til solen
lukker ruden og
holder tæt for
længslen efter 
regn der 
ikke vil

En fastholdt
plante på vagt
bag et fremskudt 
gardin

En flosset snors
forsøg på at
være lige


 

          "),
("Perler på snor",206158462976,"Stjernetæpper

Fra et levet
liv rakte hun sin
hånd og strøg 
en kind der
ikke vidste hvad
tårer var
før nu
hvor forårssolen
bredte stjernetæpper
over dækkede
ruiner

Lysets lange sommer
synger evighedens
vækst over aldres
fælles smerte
                  "),
("Perler på snor",17230201860,"Trofast i ansættelsen

Nyudlært lagde hun
til ved øretævernes
holdeplads og
forankrede sig til
pligtfølelsen der
40 år senere slap
hende løs med en
kort avisomtale
som hun klippede
ud og gemte i sin
håndtaske
          "),
("Perler på snor",223371854336,"Vestalinde


Stående udenfor
gruppen
i mit eget rum
bemærker jeg
den livligt talende
kvinde der har
alles opmærksomhed
Jeg hører hendes
lyse stemme
uden at fange
ordene
og bliver løftet af
hendes latter
gennem den grå dag
Hun tager afsked
med sit publikum
vender sig og går og nu
ser jeg hendes ansigt
idet hun går forbi
som en søjle af angst
en sorgens apati
en grå forryktheds vestalinde
Bag hende ler de endnu
og sender lange blikke
efter hendes ryg"),
("Perler på snor",524288,"Vibeskrig

Lyset over
engene fortætter sig i
et vibeskrig der
sønderriver bekymringernes
fortæppe og sætter
naturens glæde
fri

Vækster lader sig
vokse gennem den
givne plan og blomstrer
for livet der høster
glædens frugt og
lader sorgen ubevogtet
tilbage



                  "),
("Skygger",42496,"Advarsel

Mange forsøger at
gemme deres
uforløste behov i
en tæt tilproppet
flaske men
oftest ender det som
en sur affære og i
enkelte tilfælde kan
der indtræde en
gæring der kan
medføre eksplosion


Marts 
                  "),
("Skygger",0,"Afmagt

Jeg rækker ud men
kan ikke nå, hvis du ikke
griber efter min hånd


Januar 2006
                  "),
("Skygger",134774784,"Anskrig

Nu står den
måge der igen på
taget og skriger
sit budskab på
himlen 
og den har jo ret
det er let at forstå men
hvad gør man 
uden at flyve?


Marts 2006"),
("Skygger",619566860290,"Babel

Vi kæmper mod
hinanden indtil vi
forstår at det der
skiller os er
længslen efter vort 
fælles udspring i 
den samme tanke


Marts 2006"),
("Skygger",1024,"Behersket

Han er absolut
monogam, men 
ikke så meget at
det gør noget


Januar 2005
                  "),
("Skygger",17179870208,"Bemærkelsesværdig

Man kan ikke se
vingerne men
glorien er der og
den sidder lige
præcis hvor en
glorie skal sidde


Marts 2006"),
("Skygger",33280,"Bittersødt

Jeg hader
Jeg hader
Jeg
hader
Hvor jeg dog hader
Hader
Hader
Jeg 
hader
Jeg hader 

så saligt


Januar 2006"),
("Skygger",695784742912,"Blot

Solen har grebet et
hjørne af
haven og flammer
den gule mur

som en lysskakt i
sindet der fordriver 
skyggernes svale ro og
blotter et sår

Så nær kommer ingen uden smerte

Hvad sørger vi 
over
Hvert øjeblik er
besjælet og de
døde lever så længe vi
har behov for at være
dem nær



Januar 2005"),
("Skygger",1477445632,"Brydning

Nu spirer håbet mod
voldens blinde
ansigt og
alle hører isen
bryde op
Det forår der
brænder gennem
tågen lader sig vel
standse en tid
endnu før alle spæde
viljer finder rytmen i 
et fælles hjertes slag


Marts 2006"),
("Skygger",1073752066,"Brødre

Den sidste krig er den
krig i sindet der
vælter muren mellem
bror og bror



Januar 2005"),
("Skygger",549756862464,"Bøn

En ordløs bøn
løfter sig fri af
min bevidsthed
tøver et øjeblik inden den
sætter af og efterlader en
tom smerte



Februar 2006"),
("Skygger",550829557760,"Dedikeret

Tænk om man kunne 
samle frugten af sin 
indsats og lægge den på 
vægten med 
bevidsthed om at
selvom det var lidt så 
valgte man dog side
- om så  skålen med ens
bidrag bevægede sig op



Januar 2005"),
("Skygger",268471296,"Diplomatisk overvejelse

Vi ved godt vi
ikke kommer videre
uden kærlighed og
tilgivelse -
og nu er det os der
har brug for
tilgivelse fra nogen
vi ikke elsker

Vil vi overhovedet frem?



Februar 2006"),
("Skygger",25820168192,"Disse vægge

Som at leve i en
tom lejlighed
Som at stå i et nøgent
vindue og lade sine
øjne taget en
tilfældig til
fange
Som at bevæge sig
mindst muligt
for ikke at
vække det
gøende ekko
Som at hænge sine
tanker på
blufærdige vægge og
sørge for at
ingen slipper ud
Som skridtene på
trappen og
dørens guillotinelyd
Som fliser på forår



Marts 2006"),
("Skygger",17230209024,"Dissonans

Graner på
tilmålte pladser
Vindens forspildte hvisken
En løber  på
stien 
Øjnene på uret
Solstrejf uden
modtager
Himlen ridset i
striber og
øjnene på uret
Musikkens mur
mod motorvejens
overgreb og tomheden
mellem vinklede
grene
Tempoet på
uret
Stien mellem
rækker
Musikken uden
rum
Granernes beregnede
vækst og pulsens 
leg med
hans hjerte


                  "),
("Skygger",549755813888,"Diversitet

Og Gud kommer til
dem i billeder der 
er deres og 
løser deres ånd af 
den bundne jord



Februar 2006"),
("Skygger",619549034496,"Domsafsigelse

Kun den
betingelsesløse kærlighed
kan afsige den
endelige dom


Januar 2006"),
("Skygger",25786615296,"Drøm

Jeg ville jo gerne men
det går nok ikke
Jeg ville jo godt men
hvordan skulle det
gå til
Det kunne være sjovt hvis
men det er nok klogt 
og jeg har det jo faktisk
rigtig  godt
Det er vel også
forkert at 
men en lille 
drøm har man da
lov til at have så
man ikke føler at
det hele er ligegyldigt
og at man for længe siden 
har fået nok


Marts 2006
                  "),
("Skygger",8589935104,"Drømmetåge

Den lyse dag er 
lang nok for
min tanke og min 
handling men
alt for kort for 
drømmene der
flyver op og 
flakser ind i 
natten hvor de 
breder sig og 
væver sære spind så 
morgenen må 
kæmpe hårdt om
vejen ud af
tågen



Januar 2005"),
("Skygger",558345748480,"Enhed

Som et stille
åndedræt
rejser livet sig
af jorden,
kæmpende,
lidende,
elskende
Som et
pulsslag
gennem tiden,
et ekko af
et fælles
hjerte


Marts 2006"),
("Skygger",8589934592,"Eventyr

Sneen vil knirke
under mine skridt
Sneen lægger et
sårbart tæppe
bestrøet med
stjerner foran mine
skridt og bag mig
bærer den mit
spor som den har
lokket fra mig i
en lang kæde af
aftryk der fortæller en
historie for den der
kan læse og
fladen foran er lige
insisterende som én
der er grebet af den
stilfærdige beretning og
ikke kan få nok



Februar 2006"),
("Skygger",8589934596,"Facetter

Et øjeblik afslører han
knægten i sig,
den unge lømmel der
udspillede drengen
Barnet er altid
nær som en flig af den
modne mand der er
begyndt at udforske
næste trins adgang til
den gamle som endnu
kun i glimt antyder sit
omfang


Februar 2006


                  "),
("Skygger",25803883024,"Feen

Hun rakte sig så
højt hun kunne nå og
hængte sine drømme
på havens træer og
buske
Hun navngav de
vilde fugle og talte
til sine blomster der
aldrig sagde hende
imod
Hun sad under
hylden en
sommeraftens tid og
hentede lys af
dens skærme mens
hun nynnede sin
melodi og langsomt
forsvandt mellem dens 
arrede grene


Januar 2006"),
("Skygger",26172456960,"Focus

Du var altid en
bifigur i mit
liv og jeg tænkte
ikke på dig som
andet men over dit
hoved står en
stjerne og lyser over
den plads du
fylder ud og vi er
mange der står i
mørket og klipper
skygger af
lyset du sender ud


Februar 2006"),
("Skygger",550024249344,"Foran

Vi bliver en
bøn på
dagens tærskel
Et skrig om
hjælp foran
lysets sandhed
Tomme for os
selv søger vi
mening


Februar 2006"),
("Skygger",549755813888,"Forladt

Hvem har revet min
himmel i stykker og 
slængt evigheden i
sølet
Kun du der skaber af
intet kan hjælpe her  
Ordet står tomt under den
lukkede himmel  mens
kulden  fletter sin
tornekrone

Bag hækken fandt jeg i
sneen en spinkel spire 




Januar 2006

                  "),
("Skygger",1081346,"Frugt

Glæden hænger 
frit tilgængelig og
lader sig plukke
- men man skal
række efter den



Januar 2006"),
("Skygger",0,"Genklang

Gennem alle tider
rækker du dine
ord og giver min
tanke stemme



Januar 2005"),
("Skygger",135299072,"Glædesvej

Du må kende
stjernerne bedre end
jeg for du danser dig
vej hvor jeg ikke
tør træde 
- eller er det sådan at 
himlen lægger  en vej for 
den der tør følge sin 
glæde


Januar 2005

                  "),
("Skygger",8192,"Grundlag

Tankerne løber
foran og sender
budskaber tilbage til
beslutningerne der
ligger parat
katalogiseret efter
sindets ønsker
En efter en findes de
frem og effektueres med
større eller mindre
energi
Forstanden udskriver en
begrundelse



Januar 2005"),
("Skygger",1030,"Grånende guld

De, der hævder, at
alder er uden betydning,
skulle spørge de unge -
de ved bedre



Januar 2006"),
("Skygger",16787456,"Guldramme

Vi rammer ind men
rammer forkert
fordi vi tror
vi er udenfor
rammen
At træde ud af
sin egen
forudsætning er
at træde der ud
hvor vi er fremmede for
hinanden og os selv og
underlægges kræfter der
ikke kender til
forhandling
Nøgne for ramme som
udklippede figurer  på
jagt efter et landskab



Marts 2006"),
("Skygger",69793220608,"Handling

Dette er
brutalt men
retten til at
være herre i
eget hus
hævdes ikke ved
blid eftergivenhed



Februar 2006"),
("Skygger",16785408,"Her

Da jeg blev træt af at
jage regnbuer, så jeg at
jeg var kommet frem og
at jeg hele tiden havde 
løbet på stedet



Januar 2005"),
("Skygger",17179902464,"Huset

Han ved der
er flere værelser men
vil ikke vide mere
Før i tiden luskede han
rundt på
skamfulde steder og
lurede ved pirrende
nøglehuller og
enkelte gange gik han ind og
hentede den 
skyld der tynger
ham til jorden
Man kan vel levet et
liv i dette rum


Februar 2006"),
("Skygger",1343227904,"Hvornår?

Når råbet på 
kærlighed og
fred
overdøver hadet og
aggressionen -
ikke før


Februar 2006
                  "),
("Skygger",558615232544,"Håb

Vi sørger over de
døde og glemmer at
le med de
levende
Tro ikke at
mørket ikke
eksisterer men
lysets magt er
større end
menneskets nederlag
 


Januar 2005"),
("Skygger",9663678464,"Interaktion

Han lægger mig
ord i munden,
fremmede ord med
en bitter smag som
jeg må spytte ud hvis
jeg ikke skal
kvæles -
Og se om ikke de har
taget farve af mig



Januar 2006"),
("Skygger",9797896192,"Kraften

Sneen falder svævende
gennem luften og
dækker gamle
fejltagelser med
uskyld
Om lidt tager
solen over og
fjerner alle omsvøb
inden livet selv
sprænger sig vej
gennem ethvert 
tilløb til
forhandling



Februar 2006"),
("Skygger",0,"L kk t p  gr nd   f   h ld

 ng n d gt    d g d
v k l rn   r bl v t f r
bl de 


Januar 2005
                  "),
("Skygger",68803362816,"Legering

Dine øjne gennembryder
mine og vi smelter sammen
i en legering der ikke
lader sig skille igen i 
dem vi var før
Selv adskilte bærer vi
hinanden med os i den 
fælles styrke der gør 
to halve hel


Marts 2006
                  "),
("Skygger",16777218,"Liv

Fortiden er levende
som vækstlag for
nutiden


Januar 2006

                  "),
("Skygger",17196647424,"Livskunstner

Han kaldte sig
livskunstner  og
talte gerne om
sin kunst men ikke
en eneste gang
nævnte han kunsten
at række hænderne
frem og tage imod det
liv vi bliver givet


Januar 2006"),
("Skygger",8725233700,"Lærketid

På et tidspunkt i
livet løber man  panden mod
spørgsmålet om
livet mening, ja
måske endda det
gordiske:
hvad skal jeg dog her
Det er som om
gåden løser sig selv med
tiden og til sidst kan
man opstille lister af
argumenter der beviser at
det ville være en
meget dårlig ide at
hjemkalde én  lige
nu hvor foråret står for
døren og familiens
små har brug for et
smil og et klap af 
en kærlig hånd
Kan du da ikke høre det er
den første lærke der synger?



Januar 2006"),
("Skygger",33562624,"Mellem

Mellemrummene er
store nok til liv
Selv den mindste
sprække tilbyder sin
mulighed 
Måske er vi de
fleste selvom vi
ikke kender
hinanden og ikke
giver os af med
statistik 
Som grå skygger
afgrænser I mit
territorium



Februar 2005"),
("Skygger",94539620384,"Mindeord

Han blev aldrig
samtidig med sig
selv og lærte da
heller aldrig sig selv at
kende fordi han så 
verden befolket med
mennesker der engang
havde haft betydning for
hans liv men som nu
enten var døde eller
havde glemt ham for
længe siden hvis de da
overhovedet havde
været klar over hans
eksistens
Sine samtidige så
han som bifigurer der
bestandig måtte oplyses om 
de autoriteter og
forbilleder han forholdt sig til
og på samme tid modtage hans
skuffelse over nutidens
manglende forståelse af
hans verdensbillede
Det var således en i
dobbelt forstand ukendt vi
fulgte til graven og
lagde til ro mellem
de to årstal der
siden hen skulle udgøre
hele hans biografi


Marts 2006
                  "),
("Skygger",16785408,"Missiler

Som var vi
missiler der
efter affyringen
gennemrejser en
forudsigelig bane
justeret af
skyttens hånd
Lukket om vort
indre
Bundet til vort
mål



Februar 2006"),
("Skygger",8690631168,"Muren

Mit hjerter er
delt for du har
bygget en mur mod
det du ikke vil 
kendes ved selvom jeg 
har brug for at
føle mig hel

- og hvem er du der
kun kan elske en halv?



Januar 2005"),
("Skygger",77376520192,"Natlampe

Det er ikke 
solen men
lampen
ikke dagen men
natten
ikke glæden men 
noget der ikke 
vil ses
Det er ikke et
ansigt men en
maske
Det er dine øjne
jeg ikke må se



Januar 2005"),
("Skygger",85899345920,"Par

Hun græder kun med
den side der
vender bort fra 
ham
Den anden er støbt i
hans billede og ser
lige ud



Januar 2005"),
("Skygger",549755813890,"Platform

Tro er det vi
står på for
ikke at 
styrte ned



Januar 2005"),
("Skygger",8728346624,"Popler

De står i skellene mellem
afrettede marker og samler på
fugle og vindens toner
De lægger skygger  over
monotone flader og tegner 
silhuetter på  vinterhimle der 
ikke har andet for
De  stille ingen spørgsmål om
livets mening men
henter det fra dybe lag og
lader det bruge sig i 
stille åndedrag der forener
jord og himmel


Januar 2005


                  "),
("Skygger",3074,"Position

Når man kryber langs
fodpanelerne får man
øje på så meget man
har overset


Februar 2006"),
("Skygger",549755813888,"Rationalisering

Hvis  vi kun kan
opfatte Gud som én der
skal stå skoleret
hver gang  livet har
ramt os med sin
realisme, var vi måske
bedre tjent med
en kollektiv opslagstavle
der både kan tage imod og
efterlyse medfølelse fra 
andre af  vores 
egen slags



Januar 2005"),
("Skygger",8589937664,"Realitetsforhandling

Principperne hænger
løst på grenen og
et efter et falder de til 
jorden for den 
første blæst
I toppen skratter en
skade sin livslyst
og hugger et
stykke af himlen så
stort den kan nå



Januar 2005"),
("Skygger",3072,"Realpolitik

Stående på
troen
glider  vi fremad
mod  tågen vi
ikke kan gennemskue
i forventning om at
komme frem til et
mål der ligner vore
drømme 
Vi kalder os
realister


Marts 2006"),
("Skygger",33280,"Regn

Hvis bekymringer var
regndråber er det en
våd morgen og
jeg kan ikke få
paraplyen slået op -
Og så beklager man
sig over at blive
gennemblødt til
skindet


Februar 2006"),
("Skygger",68786586624,"Rejsekammerater

De følges ad
de to og
det går jo fint så længe
hun går foran og
trækker og
bagved og
skubber på


Januar 2005


                  "),
("Skygger",32768,"Sat til vægs

Jeg har set
skriften på væggen og
jeg kan ikke 
få den væk
Det gik let nok med
pråsen og den
vippende vognstang
men dette lader sig
ikke fjerne -
jeg må simpelthen stille
mig for


Januar 2005
                  "),
("Skygger",1081344,"Skamløs

Glæden ved godt
hvor den vil hen
i dag og 
jeg byder mig
skamløst til



Januar 2005"),
("Skygger",17179870208,"Skjul

Han er hverken i
ordene eller i
mellemrummene men
på sikker afstand
bag  ved




Januar 2005"),
("Skygger",17179869184,"Skribenten

Han har lagt sig i
læ bag sine ord og
hører livet fare over  
uden at se op

Da alt er roligt
lister han frem og 
samler en
håndfuld spor som

han omhyggeligt
lægger i pres mellem
telefonbogens blade



Januar 2005"),
("Skygger",25824453125,"Skygger

Solen er der og
en lærke
Vejen er der med
kirken for enden og
gårde og huse med
mellemrum
Huset med angsten et sted 
bagude og vinden og
hans lyse hår
Skyggen er der
vokset fast til
hans fødder,
fjorden på hans 
ene side og
bugten på den anden  så 
landet er som en 
finger med en vej  og
en dreng med hans skygge og 
skyggen indeni som
han ikke kan se og 
strengen bagud der er
bundet til angsten og
moderen der altid 
græder og han
går til  strengen strammer og
holder ham fast og
trækker ham tilbage
selv da han er blevet en 
voksen mand og
hans skygge har  vokset sig
lang og tynd nu
solen er ved at gå
ned og lærken er
fløjet sin vej



Januar 2006


Samhørende:

Damen
En sommerdag
Farvefilm
Onsdag
Guld
Hønsegård
Juleaften
Skygger

Alle fra samlingen Billedbogen
             "),
("Skygger",16785920,"Smut

Den lige vej er
beredt
Sidevejene 
åbne
Jeg var altid
en smutter



Marts 2006"),
("Skygger",549755815938,"Splid

Det er ikke Gud der
splitter os
Det er os der tror vi
kan bruge Gud mod
hinanden



Februar 2006"),
("Skygger",558479974404,"Sporet

Vi går gennem
landskaber under
himle der strømmer
igennem os og
farver vores
bevidsthed som
tiden der ikke er
tid men livet selv
som det farer gennem
kroppen og
beliver processer
uden for vores
rækkevidde og 
vi går gennem vore
aldre som var de
landskaber der
tager imod men hele
tiden fordrer at vi
går videre ad
sporet der bærer
vore skridt som en
tilgivelse vi ikke
vil vide af
Vi går under himle vi
aldrig når frem til
skønt de er som en
del af os selv
gennem aldre vi ikke
kan nå men som er der når 
vi vender os og ser tilbage på det
netop forladte inden vi
går frem mod
himlen indhyllet i
os selv



Februar 2006"),
("Skygger",558345783298,"Spot

Man kan ikke spotte Gud
Gud er uspottelig
- men man kan
spotte mennesker og
det er slemt nok


Februar 2006"),
("Skygger",1073741826,"Spotlight

Den ondeste smerte
gemmer sig bag
uvisheden
Den nøgne sandhed
kan man måle sig med


Marts 2006"),
("Skygger",0,"Sted

Spor i den 
frosne jord
Et laksegls stivnede
aftryk
En lige linie
mod horisonten der
skjuler en 
fremmeds sted


Marts 2006
                  "),
("Skygger",8589942784,"Tolkning

Vinterfugle på vej i
et øde der synes at
strække sig hinsides
flygtigheds grænse

Levende arabesker i sne

Vinteren synger sig ind på
mit sind og tolker de
tegnede drømme

Også vore spor
lader sig vel tolke som andet end
aftryk på  den 
snedækkede  vej


Februar 2006"),
("Skygger",25769804800,"Tung

Han er sådan en
trist én og det
forstår man godt
når man oplever de
tunge ord han
bærer rundt på
Åbner han munden
falder de til jorden og
ligger der uden at 
kunne være med
Prøver man at samle
ét op får man brug for
begge hænder



Januar 2005"),
("Skygger",134219776,"Udtryk

Børnene ler så længe
latter kan fås men

hvad skal blomsterne
stille op hvis
farverne bliver
forbudt

Mens lyset tændes på
dagens alter
mødes vi af den
blide duft





Februar 2006"),
("Skygger",283602059264,"Udveksling

Bladene vender sig
for vinden og
lader deres budskab
flyde frit gennem
luften
 
Bladene henter deres
kraft i dybe lag og
lader jordens
gave  strømme til
det rodløse

Bladene ånder 
lysets kraft og
giver den i bytte
for klodens visdom



Februar 2006"),
("Skygger",1073743874,"Undskyld

Kun den der er 
skyldig kan 
påtage sig
skyld -
alt andet er
politik


Februar 2006"),
("Skygger",8192,"Uudsigeligt

Det er som vinden der
meddeler sig uden ord
Som sætninger der 
bærer et skjult budskab
Det er som et flygtigt
øjekast der sætter en
verden i brand
Som svævende snefnug
der lægger sig tæt om
en åben smerte
 
Marts 2006"),
("Skygger",32768,"Vammelt
 
Der lå et smil på
gaden som nogen havde
tabt 
Skønt det så lidt
krøllet ud klistrede jeg
det på men fik det
hurtigt af igen fordi
det var for sødt
Min egen smag går 
mere i retning af 
syrligt



Januar 2005
                  "),
("Skygger",134250500,"Veteran

Præcist som en
frugt -
med alderen bliver
skallen sej og
afvisende men lykkes det at
komme indenfor kan
man finde en
forbavsende frisk
sødme



Februar 2006"),
("Skygger",553379692544,"Vished

Vi står foran
lysets ansigt og
kan ikke
bære tvivl
I en stofløs bølge
flyder visheden over
horisonten og
oversvømmer alt på 
sin vej
Lange skygger kalder
på mørket der for længst
har givet op for den
ordløse sandhed der ikke
står til overvejelse men
løfter sig 
uhindret over
dagens tærskel



Februar 2006"),
("Skygger",16785408,"Væggene

Væggene står ret og er
ikke blot vogtere, 
afgrænsere eller
beskyttere men
mulighed for koncentration
af en væren der
kunne se ud som en
ligegyldig plet på et 
åbent spørgsmål



Februar 2006"),
("Skygger",6,"Øjebliksbillede

Idet man fokuserer på
sin alder møder man sig selv et
sted på livslinien uden at kunne se
slutpunktet men ældre end man
nogensinde har været før


Januar 2006
                  "),
("Stengrunds tørst",0,"1980-83"),
("Stengrunds tørst",163259088929,"Abepedersen


Da han solgte kroen, 
solgte han 
aben med. 
Abepedersen 
der sad i sit bur 
bagest i haven 
og var søndagsturens 
attraktion. 
Kromandens drikkende 
abe, 
der rystende på 
sine gamle 
hænder 
bad de leende 
turister 
om en lille en 
til halsen. 
Bagefter kvitterede 
den for skænken 
med alle drankerens 
genkendelige 
reaktioner, 
der her i aben 
virkede ufarligt 
morsomme. 
Den nye ejer 
havde ondt af aben 
og tænkte egentlig på 
at afvænne den, 
men indså hurtigt
at Abepedersen 
var mere end 
attraktion. 
Abepedersen 
var skæbne 
og mere end det. 
Noget uafvænneligt 
der gik sine egne 
veje, 
og derfor fortsatte 
aben sit liv 
i buret for enden af 
haven. 
Abepedersen fik en dag 
besøg 
af sin gamle ejer. 
Gensynsglæden var stor 
på begge sider. 
Dens reaktion så 
formidabel 
at den gjorde indtryk 
på de to besiddere af 
Pedersens 
kærlighed 
og hurtigt dukkede 
det uundgåelige 
væddemål op, 
der skulle betyde 
katastrofe for 
en indespærret 
hengivenhed. 
Prøven blev arrangeret 
på den grusdækkede vej 
udenfor. 
Én ejer 
på hver side 
og Abepedersens oldingekrop 
fastholdt 
i midten 
til signalet blev givet: 
Slip Pedersen løs. 
Lad ham vælge side. 
De sagde bagefter 
den skreg så højt, 
at de kunne høre 
det en kilometer borte, 
men det var få der 
så abens krop 
forsøge at vælge side. 
Han smed sig ned, 
Abepedersen, 
og gav sig til at skrige. 
Var sanseløs væk 
i nådig krampe, der 
efter en tid 
gjorde kroppen 
stille. 
Hvad fanden gik der af ham? 
sagde de bagefter. 
Hvad skulle det til for? 
Men alle vidste, 
uden at kunne give det udtryk 
i ord, 
at de havde oplevet noget, 
de aldrig skulle glemme. 
Kærlighedens sliden 
et hjerte i stykker. 
Abepedersen døde ikke alene, 
men han døde 
forgæves. 
Han blev ofret 
for en nysgerrighed, der 
ikke stod mål med hans 
drankersjæls tørsten 
efter noget, der var 
større end ham selv. 
En uslukket tørst 
efter noget, han 
aldrig rigtig fik, 
men bare anede 
i sit bur 
for enden af 
haven 



                  "),
("Stengrunds tørst",4294967296,"Aften


Det er slut 
for i dag. 
Mørket lister ud 
af jorden 
og siver 
opad 
hvor naturens 
farver 
gløder 
som essensen
af dagens 
stræb. 
Og langsomt 
tændes 
stjernerne 
der synger 
en evighed 
ud over 
dækkede 
verdener 
ingen kender.


                  "),
("Stengrunds tørst",137472540672,"Alvor


Livets alvor
maler landskabet 
vemodigt, 
mens det strømmer 
forbi min undren. 
Livets alvor er vist 
aldrig helt alvorlig, 
når det er mig, det 
rammer. 
Var det en anden, det 
gik ud over, 
havde det sikkert været 
alvorligt, 
men når det er mig, 
der er ramt, er jeg lidt i 
tvivl. 
Det rigtige liv 
kommer nok ikke 
til sådan en som mig, 
og ulykken er til 
at bære, 
så længe de andre 
ikke bemærker 
noget. 


          "),
("Stengrunds tørst",17825792,"Beredelse


Det vælter op af jorden,
jeg brækker løs på
med min spade,
så det værker
i kroppen længe
efter.
Sten og ler,
smådyr og
underlige formationer,
der bare får lov til
at vende rundt en gang
uden at blive kigget nærmere på.
Bare ned med dem igen.
De skal blive til
grøntsager
næste år,
når jeg engang
er blevet færdig
med alt dette her,
der altså bare er
at berede jorden.
Næste forår
skal der drysses
lidt krymmel ned
og så vil det alt sammen
ganske af sig
selv
blive formet om
til gulerødder
og radiser,
til porrer og rødbeder
med hver deres duft og smag.
Gudved om der kan vokse
sådan en redelighed frem
af mig,
hvis jeg giver mig til at grave
i mig selv og
lader
alt det formløse
komme frem i lyset
og vende en tur
under forstandens,
følelsens og fantasiens
sollys.
Kommer der også
et forår for mig,
så Gud give
at jeg må vide,
at berede jorden
nu.

          "),
("Stengrunds tørst",206175240192,"Betroelse


Det bliver 
pludselig 
for meget for dig. 
Der går ganske enkelt panik 
i dig, 
da jeg tager dig med 
indenfor, 
hvor vi med et står 
overfor 
virkeligheden. 

Vi har længe 
hygget os 
udenfor 
i ærlig tosomhed, 
men da vi åbner døren 
til virkelighedens 
ramme lugt, 
vil du ikke med. 

Vil ikke vide mere. 

Så lukker vi døren 
igen. 
Vender tilbage 
til den gamle 
position. 
Men hyggen er borte. 
Og skamfulde falder 
det os svært 
at se hinanden 
i øjnene. 

Vi taler sammen.


                  "),
("Stengrunds tørst",33587200,"Dødvande


Hver 
eneste 
dag 
er 
som 
dagen 
før 
skønt 
jeg 
hver 
dag 
håber 
at 
dagen 
i morgen 
er 
ny. 
Forberedt 
på 
det 
uventede 
venter 
jeg 
utålmodigt 
den 
ny 
dags 
grå 
bag 
nattens 
åbnede 
porte. 


          "),
("Stengrunds tørst",68753031168,"Drøm


Vi sender 
vore inderste 
tanker 
anonymt 
til hinanden 
fordi vi ikke 
vover 
at adressere 
den kluntede 
drøm 
der ligger nedenunder 
de lag 
vi tør vedkende os
over for os selv.



                  "),
("Stengrunds tørst",8725209088,"Dugdråbe


Sekundet 
er som 
en dugdråbe. 
Overvældende 
afrundet. 
Underfuldt 
fuldendt. 
Formspændt 
i sig selv. 
Med mulighed 
for lysets 
tindren, 
med mulighed 
for fald. 
Sekundet er 
som en dugdråbe, 
du drikker 
af et blad.

                  "),
("Stengrunds tørst",16809984,"Ejer


Hellere 
end at lege 
turist 
i mit eget land 
vil jeg tage det i 
besiddelse 
ved en bevidst 
flyttedagsbeslutning.
Fylde hele 
pladsen 
ud 
og erklære 
området 
for 
uindskrænket 
mit 
uden at 
skulle betale 
told 
ved grænsen.



                  "),
("Stengrunds tørst",4596989952,"En stjerne


En stjerne er tilbage
mellem de
flossede skyer.
Slukkes også den
vil mørket brøle
kvælende
og suset fra
poplen
i min have
vil flyde
usynligt
mod husets
nøgne
mure.

                  "),
("Stengrunds tørst",167772160,"Ensom


Det ensomme menneskes 
drivværk 
er længslen 
der langt inde 
bag kødet 
nervøst fingererer 
med håbet 
om andre tider. 

Selv på dage 
hvor skuffelsen 
tilsyneladende 
har slukket 
livsviljen 
trækker længslens 
fjeder 
livet pulserende 
gennem årerne.


  
Anvendt af Cafeteatret 2005 
i forbindelse med 
skuespillet Under bæltestedet 
af Richard Dresser                      "),
("Stengrunds tørst",33554432,"Ensomhed


Musikken
spiller i baggrunden
dybt på bunden
af tankernes hav.
Den klager skæbnesvangert
bruser
skriger
vildt
i nød
inød, i nød, i nød.
Så tier den
der høres kun
en sagte hulken
fra fagotten.
Nu tier den
og ensomheden kommer snigende.
Trøstesløsheden
Ørkensand
en bleggrå
skyfri himmel
En lige horisont.


          "),
("Stengrunds tørst",33587200,"Flugt


Langt herfra
ligger byen 
med mange 
menneskers drømme. 
Byen har 
så mange 
universer 
at mit 
alene 
mellem 
bakkerne 
blafrer 
uvirkeligt 
efter ekko. 
Deres drømme 
og mine drømme. 
Mit univers 
og deres. 
Jeg sulter 
efter jer 
drømme. 
Jeg tørster 
efter jer 
umistelige universer 
mellem mine 
bakker. 
Jeg vil drukne 
i folkehavet. 
Opsluges 
af massernes intethed 
for ikke at føle 
min egen 
smerte. 


          "),
("Stengrunds tørst",17196646400,"Forbillede


Du er 
et billede. 
Ikke af dig 
selv, 
men af en,
der passer 
til dine 
tanker. 

Dit billede 
lyser over 
dig og 
dækker dig 
til med 
en lerøjet 
livløs 
maske. 

Maskekampen 
begynder, 
når du opdager den,
og finder den 
kæmpende 
mod dit jegs 
inderste 
interesser. 

Den kæmper 
mere voldsomt 
end du, 
der er den, 
der skal 
gøre bevægelse. 

Stivnet 
klæber den 
til dig 
og dræber 
alle 
udtryk. 


          "),
("Stengrunds tørst",77326221824,"Frelser


Så kom da
siger du venligt.
Så kom da 
og lad mig hjælpe dig.

Jeg har ikke råd 
til at sige nej 
og sidder med et 
i saksen. 

Du viser mig den
sandhed, 
der har lokket mig 
herhen, 
i alle dens 
rige facetter. 

Jeg kan ikke lide det, 
men har ikke råd 
til at vise det. 
Jeg må tage imod 
din skamløse 
åbnen mig op 
under sandhedens 
knivskarpe spotlight. 

Sådan, siger du. 
Sådan. 
Nu kan du være fri. 
Ja, siger jeg. 
Ja. 
Jeg må tænke over det,
og ved med hele mit 
væsen: 
Jeg må lukke mig. 
Lukke mig. 
Lade som om 
det aldrig har været. 
Jeg må for altid 
undgå dig. 
Hade dig. 
Fornægte dig. 

Det koster dyrt 
at være sin næstes næste.



                  "),
("Stengrunds tørst",1090551808,"Frygt ikke


At frygte 
livet 
er 
at frygte 
sig selv 
fra øjeblik 
til øjeblik. 
Intet 
er 
at ræddes for 
hvor du 
er hel tilstede 
i din 
inderste vilje 
til 
sandhed.

          "),
("Stengrunds tørst",17334272,"Fuglene


Fuglene dukker frem 
af buskenes sikkerhed 
nu jeg bliver stille 
og sidder med 
spaden over knæene og 
ryggen op mod muren. 

Her er så stille 
langt ind i mig, 
hvor fuglene i mig 
forsigtigt 
stikker hovederne frem 
og ser sig om. 

De vover sig længere frem, 
mine fugle og buskenes. 
Sender mistænksomme 
øjekast i alle retninger. 
Sidder lidt 
inden de nærmer sig 
beddet, jeg lige har gravet.

Drejer jeg hovedet 
er de væk igen 
på blafrende, 
hurtige vinger. 

De evigt 
flygtende 
fugle. 


                  "),
("Stengrunds tørst",69826805760,"Grovæder


Du spiser 
spørgsmålstegn.
Du spiser 
alt for mange 
i stedet for 
at sætte dem. 
Det irriterer mig, 
for det giver mig alt 
for lille 
mulighed for 
at komme tæt 
på dig. 
Du lukker dig 
med spiste spørgsmålstegn. 
Bøvser af mæthed. 
Bobler over med forklaringer, 
der afløser hinanden 
i en uigennemtrængelig 
rustning, 
der værner dit 
ømme indre.



          "),
("Stengrunds tørst",68770890240,"Helvede


De siger
til dig, 
at det er de andre,
der er 
Helvede, 
men lyt ikke 
til dem. 

Helvede 
er 
ikke 
de andre, 
men dit eget indre, 
omgærdet 
af dine egne 
fordomme 
vogtet af 
dig selv. 

Du er din 
egen 
Satan, 
når du vender 
al din naturlige 
livsvilje 
indad til 
selvkontrol
på et falsk 
grundlag. 

Det er ikke 
de andre, 
der sidder inde 
med nøglen 
til dig. 
Du har den 
og bestemmer 
selv 
hvornår, 
hvordan og hvorfor 
du vil have 
indgangen 
åbnet 
til enhver 
muligheds 
helvede eller 
paradis. 

De andre er 
medrejsende 
og side om side 
vogter I private 
verdener, 
der burde gøres 
frugtbart 
tilgængelige, 
ikke til fælles 
eje, 
for intet er 
i grunden 
fælles 
i denne verden, 
men tilgængelige 
for det liv, 
der tænker os 
alle. 



                  "),
("Stengrunds tørst",50364416,"Hulen


Dagene er lange
og grå
med gule og brune
skygger
der haster
hurtigere og hurtigere
forbi
altid forbi
min hule.
Tøver en
uden for
ræddes jeg
for et møde
og lader
som om
jeg sover.



          "),
("Stengrunds tørst",134217728,"Hvad venter vi på?


Vi venter 
på 
forløsning.

Venter 
på 
resultater 
af vores 
væren. 

Facit 
rummes i 
nuet, 
der kan 
udstrækkes 
uendeligt 
til begge 
sider 
af sig selv 
og omspænde 
evigheden. 

Længst 
stivnede 
sorger, 
kommende 
kriser 
og kras 
hverdag 
har intet forsvar 
mod nuets 
mulighed, 
der evigt 
ligger 
åben 
for os. 


          "),
("Stengrunds tørst",68719476736,"Hvorfor


Hvorfor dør vi så tit
af at leve med andre?
Mennesker omkring en
får pludselig automatkarakter
og taler med robotters stemmer.
Deres eget jeg
er som kørt ind
på et spor
der går regelret ligeud
og ender
et sted
bag horisonten.



          "),
("Stengrunds tørst",85916123400,"Hvorfor Robin?


Jeg følte mig
uvirkelig,
sagde han.

Det vil sige,
det var ikke
de ord,
han brugte,
for han kendte 
dem ikke.
Han sagde:
Det var åndssvagt.
Men ruden var
virkelig
og den der
smadrer andres
virkelighed
kan ikke selv 
være
uvirkelig.
I splintret glas
fødtes hans
virkelighed
for lynhurtigt
at dø under vores
spørgsmål.
Hvorfor Robin?



                  "),
("Stengrunds tørst",4596989952,"Ilden


Nu dør ilden hen
for anden gang
i nat
i pejsens mørke
hule.
Den grå stilhed
lister sig ind
på mig og
gør mig alene.
Får mig til
at huske
smertelige oplevelser
jeg troede glemt
og besejret.
Rystende lægger jeg
et stykke træ
på de ulmende gløder.


                  "),
("Stengrunds tørst",549773639680,"Indsigt


Meningen 
med min 
tilværelse 
er måske 
netop der 
hvor jeg ikke
har søgt: 
I lydig troskab mod 
det der længe har 
været mit. 

                  "),
("Stengrunds tørst",68753063952,"Isblomster


Tiden har slidt os. 
Elimineret 
det center 
der skulle styre 
sammenhæng. 
Selv når vi 
danser 
er vi 
alene. 
Onanerer 
bag tilværelsens 
ruder 
som uvægerligt 
fryser til 
af grænse 
for et 
enkelt 
menneskes varme.



                  "),
("Stengrunds tørst",16809984,"Jet


Lyden 
af en enlig 
jager 
buldrer gennem 
aftenmørket 
og ripper op 
i dagens slumrende 
ængstelse 
for en 
fremtid 
der ikke vil 
konfronteres 
med mit indres ønsker. 
Lyden anslår 
et tomrum i mig 
der vibrerer 
af spørgende 
tomhed. 
Jeg er 
en skal 
om glødende 
øde 
der smerter 
efter at 
fyldes 
med tro 
på noget 
der er større 
end mine 
egne drømme.


          "),
("Stengrunds tørst",68736286720,"Kadavere


De andres 
billeder 
virker 
skræmmende 
i deres retoucherede 
perfekthed. 
Afsluttethed. 
Enhed. 
Ens egen 
famlende 
mangfoldighed 
bøjer ydmygt 
hovedet 
når et overmalet 
kadaver 
stolt 
skrider 
forbi. 
Det er ærgerligt
hvis rige 
muligheder 
forspildes 
af benovelse.


          "),
("Stengrunds tørst",274898878464,"Korsvejen

Og så drog de ud
i den store verden
og skiltes ved
korsvejen
under den gamle egs
krone.
Som tegnbærer
stak de deres
eneste eje,
den blanke dolk,
ind i træets
hårdt prøvede
stamme.
Hvor mange ynglingedolke
bærer du spor af?
Hvor mange rustne 
blade har du ædt
med din krop?
de unge fyres
knyttede rygge
har trodset dit
sfinksvæsens
seen.
Hvor mange vendte tilbage
for at søge sin
tabte broder?
Hvor mange vendte tilbage
til stormende
manddomsmøde?
Hvor mange
blev borte
bag bakken?

De drog ud.

Hvor drager du hen,
du gamle
væsen,
der lever
af fortidens
eventyrkraft?
Spyt ud.
Spyt ud med din viden,
du har suget til dig
med talløse sanser.
Del ud af din viden
til dem, der
skilles
for aldrig at
mødes hos dig igen.
Råb et opmuntrende
ord til dem, der skal ses
igen.
I det mindste
et håbefuldt ord
til dem,
der trodser angsten.
Kan din viden
aldrig vækkes
af sin århundredgamle
søvn?
Er det derfor, du tier,
eller råber du sandhed
med flænset stemme,
vi ikke kan høre
i nutidssøvn?


                  "),
("Stengrunds tørst",68753031168,"Længsel


Vi længes efter 
at række hinanden 
hånden 
og trænger 
vedgud 
til det 
men selvom 
vore hænder 
når hinanden 
er det som om 
længslen 
stadig piller
ved 
et eller andet. 

          "),
("Stengrunds tørst",135266320,"Lad blomstre markens blomster


Giv livet en chance i dag 
fredag. 
Giv livet en chance 
nu. 

Ikke om lidt. 
Ikke i morgen. 
Om en uge. 
En måned. 
Et år. 

Giv livet en chance
i dag 
fredag. 

Nufredag. 

Nu. 

Kun øjeblikket 
har magt over 
fortiden. 

Kun øjeblikket 
er øjeblikkets 
mulighed. 

Kun øjeblikket 
bestemmer 
fremtiden. 

Vibrerer af 
liv, der kan 
dirigeres 
i ringe indad 
eller udad 
mod hvert 
sekunds egen 
evighed. 

Du bestemmer 
ingenting, 
men er under 
øjeblikkets 
kærtegn. 



                  "),
("Stengrunds tørst",4296048640,"Lettelse


Nu regner det
bare
og stormen er
standset.
Vidunderligt stille
derude
hvor jorden
har åbnet
sig tyst
for regnen
og tusmørket
hvisker
godnat.

                  "),
("Stengrunds tørst",9009496080,"Markbrand


Markerne brænder 
i mørket 
Knitrende røde 
valmuer 
vugger 
vildt i mørket

Markerne tømmes 
for mørke
Tømmes for 
tomme tanker, 
der har udspillet 
deres tid

Markerne oser 
mod himlen
Oser det udbrændte 
selv 
op i de 
tindrende
stjerner



                  "),
("Stengrunds tørst",25811780096,"Men ikke et øjeblik
glemte jeg
mig selv.


Jeg er i et 
betonhus 
med døde 
vindueshuller 
i den grå 
facade. 
Jeg kigger 
varligt ud. 
Lever drømmende 
i vindueskarmen 
og betragter 
dem 
der går forbi
på gaden. 
Deres skridt 
runger hult 
mod mine 
mure. 
Ser de op 
trækker jeg 
mig tilbage 
i værelsets 
mørkeste krog 
til de er 
passeret forbi. 
Læne mig ud 
tør jeg ikke 
af angst 
for at falde. 


                  "),
("Stengrunds tørst",8589934592,"Nu


De ord 
vi siger 
i dag 
er 
velsignelse 
over 
det liv 
vi skal leve 
i morgen. 
De tanker 
vi tænker 
i dag 
er født 
af opbrugte drømme.
Nu 
er det 
tid 
at forme 
det formløse. 
Så 
morgenens 
gave.


                  "),
("Stengrunds tørst",16809984,"Ornamenter


Man kan føle angst for 
at det man tror på 
er løgn 
og flugt fra 
virkeligheden. 
At det man stoler på 
er sat op 
af en selv. 
Et selvforførelsens spejl 
der afspejler 
de strømninger 
der farer gennem en 
som man ønsker 
at se dem 
fortolket 
til logiske 
skønne ornamenter.

          "),
("Stengrunds tørst",1048576,"Sådan

Du skal leve af
at hvert
sekund
er hver
tidsalders
ny begyndelse.

Du skal håbe på
at hvert
sekund
er hver
livsgnists
meningsfyldte
gnistren.

Du skal dø på
at hvert sekund
er hver
tidsalders
frugtbart
frodige høst.


          "),
("Stengrunds tørst",8606744576,"Salami


Dage 
der smager 
af smerte, 
nætter 
der 
lugter 
af ængstelse 
kører 
i 
cirkelbane 
med mig. 

Jeg synes 
ofte 
mod mig. 

Dage 
der ikke 
er mine 
dage 
bruger 
mit liv 
skive 
for skive 
uden at spørge 
om lov. 

Jeg vil 
ikke 
men lader mig 
alligevel 
spise op 
i papirstynde 
skiver. 

                  "),
("Stengrunds tørst",68769808896,"Selvoptaget


Selvoptaget?
Selvfølgelig
er han
selvoptaget.
Det er jo
hans selv
der står
på spil.

Selvfølgelig
lukker han sig.
Hvad skulle
han ellers
gøre,
når vi forsøger
at kvæle
hans dyrebareste 
væren sig selv 
med vores
opskrift
på
normalitet.

Vi pløjer
hans forkvaklede
ager
med teknikkens nyeste
plove
og ænser ikke den
sammenhæng,
vi vender op som
tilfældige
potteskår.


                  "),
("Stengrunds tørst",68736254464,"Sjælesorg


Du læner dig 
roligt, 
alt for roligt, 
tilbage mod væggen. 
Bøjer det ene ben 
ind under dig 
og lytter, mens du ikke ser 
på mig. 
Det er jo også mig, 
der skal tale ud, 
og derfor kan min 
krop ikke indtage 
din stilling. 
Og mit sind ikke 
se roligt til. 
Jeg krummer mig 
sammen. 
Jeg krummer mig 
sammen 
under din lampe, 
du beredvilligt 
har tændt 
alene for mig. 
Og så lytter du. 
Lytter som 
bare helvede. 
Eller lader du bare 
som om. 

Det er jo det, du 
er her for. 
Det er det, vi er 
her for. 
Du, for at lytte 
og jeg, for at 
tale al min 
opsparede fortvivlelse 
ind over dit spisebords 
hverdagsmærkede plade. 
Og så kommer de. 
Ordene. 
Jeg brækker dem op 
og tømmer dem 
ud i svinske 
kaskader, 
og er du forbavset, 
kan man i hvert fald 
ikke mærke det. 
Og har du hørt det før, 
hører det vel med 
til bestillingen. 
Tænker du på dig selv 
imens? 
Hvad du skal sige om lidt, 
når min krampe 
omsider 
tager af? 
Du kan godt spare dig,
for jeg vil ikke 
kunne høre det. 
Jeg er alt for træt 
og har fået 
nok, 
når det hele 
er læsset af. 
Hvad skulle du også sige, 
jeg ikke har hørt før, 
og gudved om du 
sidder inde 
med nye 
kombinationer.
Det gør du vel ikke 
- håber jeg. 


                  "),
("Stengrunds tørst",77410140160,"Skilsmisse 

De mure, der vokser op omkring os, 
er ikke mure, 
men udfordringer, vi ikke 
vil tage op

Den afstand, der begynder at skille 
os, 
er ikke afstand, 
men liv, 
vi ikke vil tillade liv

Det had, der skæmmer vores kærlighed 
er ikke had, 
men længsel 
efter en fremtid, 
vi ikke tør 
tage imod

                  "),
("Stengrunds tørst",8725463040,"Snefnug


Hvem lytter 
til de faldende
snefnug, 
når de yndefuldt 
glider 
til jorden? 

Hvem går på vandring 
i lette krystaller, der 
åbner sig 
indad, som 
sindets 
krogede gange? 

En verden af sne 
ligger brak 
under solen.

                  "),
("Stengrunds tørst",0,"Spadseretur


Lys ligger dagen hen 
i stilhed. 
Bakkerne 
skærer 
himlen over 
og strækker 
liv 
et stykke 
over jordens 
grobund. 
En stæreflok 
piler gennem 
mit sind. 
Punkterer 
himmelvælvingen 
der lader 
væden plaske 
ned over 
min nøgne 
pande



          "),
("Stengrunds tørst",77309411328,"Stilstand 


Vi gør krav på hinanden. 
Ikke med udtalte ønsker
og henstillinger, 
men med livsmønstre vi 
ikke vil bryde. 
Med eventyr vi ikke 
vil lade gå 
deres egen gang 
i vores sirlige stuer.

Vi nægter at hoppe 
i brønden 
og lande på 
engens bløde græs, 
hvor livet kalder på 
os i former, vi ikke 
vover at kende 
fra vores hverdags 
bastioner. 

Vi nægter hinanden 
retten 
til at vokse 
personligt 
og forstener derfor 
kollektivt 
til hinandens onde 
stedmødre. 

Ser ikke 
hvor gemene vi er 
i vores blide 
ømhed. 



                  "),
("Stengrunds tørst",15051260416,"Stævnemøde med sig selv


Den udvendige stilhed 
er blevet for stærk 
til at døve 
en indre 
skrigen 
fra forrykte munde, 
der næsten 
synligt 
brøler deres 
hæse bønners gift 
gennem mine årer. 

Som i en eddask
ormegård 
vrider vrængende 
væsener 
en stadig stærkere
søvnløshed 
frem af min 
hverdagssyge 
særhed.

Lytter jeg, 
lytter de. 
Taler jeg, 
overdøver de mig.
Er vi hinandens 
fjender 
eller 
kalder de 
på en solidaritet 
i mig, 
hvis sande 
ansigt 
jeg ikke kender? 

Tør jeg? 
Tør jeg 
tie? 
Tør jeg være den
første, 
der bryder 
den lyttende
stilhed 
og bliver 
stille? 

Helt stille 
uden at lytte. 

Bryde lytternes 
lurende 
lytten. 

Hvad er det, 
jeg ikke 
må høre 
eller er der 
muligvis noget, 
jeg ikke vil 
høre 
med hele 
mit væsens 
jeg 
som indsats? 

En uundgåelig 
forryktheds 
sandhed? 

Sandhed. 
Hvad er sandhed? 
Er sandhed andet 
end bøn om nåde? 
Er forsvarsværkerne 
andet end 
kulturens 
døde arvegods? 
Forlængst afdøde 
ideers 
rådnende kroppe? 

Livet 
venter ikke. 
I livets skole 
er intet andet 
frikvarter 
end det, 
du lyver dig til. 

Hvad du flygter fra, 
flygter du til. 

Urfjeldets 
kalden. 

Stengrundens 
tørsten 
i dig. 

Stenen 
ligger på 
stranden 
under klinten med skoven 
som idyllisk kulisse 
bagved. 

Flad og så 
stor 
at tidevandet 
aldrig anfægter
dens væren 
kolos. 

Jeg var her engang. 
Siden har den 
ventet 
på mig. 
Ventet på 
Robinson, 
der udmattet 
kravler 
i land, 
fordi landet 
trods alt 
er bedre end havet. 

Med hænderne 
skygger jeg 
for øjnene 
og fanger 
et par lystsejlere 
tæt under kysten 
på den anden side
bæltet. 

Det er vel 
utænkeligt 
at du og jeg 
har mere 
og andet at gøre 
end det 
at være 
sten og mand 
lige nu? 

At du og jeg, 
at sten og mand 
ligefrem 
ligner hinanden 
og begge venter 
på en slags 
forløsning? 

At sprækken, der arrer 
din rustne kind, 
svarer til en 
lignende 
spaltning 
inde i mig? 

Gør det ondt 
på en sten, når saltvandet
siver ind 
eller er den så 
hård, 
at den intet mærker? 
Eller 
bliver smerten 
uden på? 

Ligner vi overhovedet
hinanden? 

Eller er vi bare 
hinandens verden 
lige nu? 

Du min
og jeg din?

Du er da i hvert fald 
min 
lige nu. 

Du er min. 
Hører du? 
Min. 

Du svarer ingenting 
og himlen over dig 
driver med 
luftige kloder. 

Stjernerne. 
Stjernerne. 
Jeg er svimmel 
under stjernerne. 
Kan ikke længere se 
dig, 
men føle dig. 
Føle dig. 
Jeg må mærke et andet 
jeg 
nu, 
hvis jeg ikke 
skal blive borte 
under stjernerne. 

Fortabes 

Du ligger der bare. 
Ligger der, 
kold, 
men vidunderlig 
stor og stærk. 

Urfjeldet 
emmer af 
dine porer 
og gør dig 
sten. 

Er jeg af en anden slags
eller ånder 
samme gnistren 
gennem os begge? 

Skoven lytter. 
Hører du 
skovens lytten 
eller lytter du efter noget andet, 
noget større, 
noget urørligt, 
uhørligt? 

Du ligger så stille,
men jeg sanser, 
at noget lever 
i dig. 
Noget, der 
har med min verden 
at gøre 
på en måde, 
jeg ikke forstår 
med min forstand. 
Har vi allerede vænnet os 
så meget 
til hinandens kroppe, 
at de taler sammen 
uden at skulle 
om ad os, 
der bare ligger.

Jeg ligger på dig.
Du bærer mig 
og sammen 
holder vi hele 
jorden 
nede. 
Letter vi på os, 
ryger det hele 
opad. 

Opad. 

Op mod 
de 
kaldende 
stjerner. 

Jeg elsker dig. 
elsker dig. 
Hører du? 
Jeg elsker dig, 
selvom jeg tæver 
din elendige 
krop 
til du bløder. 
Jeg elsker dig 
eller 
er det mig, 
der 
bløder? 

Du lyser så 
stille. 
Du gløder så 
lydløst, 
at jeg ikke 
ville have opdaget det,
hvis ikke dagen 
havde 
vækket mig. 

Sover du? 

Sover du nu? 
Jeg kan mærke, du 
trækker vejret 
på en måde, 
jeg aldrig har 
hørt før. 

Du ånder. 

Jeg lader lyset lege i dig.
Jeg lader lyset lege med dig. 
Jeg elsker dig. 

Morgen. 
Havet. 
Himlen og skoven. 
før i tiden 
kunne man tale om det. 
Nu findes det dårligt 
mere. 

Morgenstemning ved havet.

Fuglene har sunget 
i skoven 
længe. 
De begyndte tidligt. 
Dengang du begyndte 
at gløde. 
Det er så vigtigt 
at fortælle det 
til dig. 
De sang 
med en styrke, 
der burde have kaldt 
dig tilbage 
fra dig selv, 
men du hørte ingenting, 
da du lå der 
og havde forbindelse 
til dem 
af din egen slags. 

Du hørte 
heller ikke mig. 

Hørte ikke 
min smerte. 

Havde du en tot hår 
et eller 
andet sted 
ville jeg 
stryge dig 
over den 
og føle mig 
nænsom. 
Alene. 

Det er tid at gå.

Det er tid 
at gå nu. 

Du har lukket dig. 

Lukket mig 
ude. 

Din arrede flade. 

Sten. 

I aften skal 
du lytte 
til stjernerne 
uden mig. 

De evige 
kaldende stjerner. 

Evigheden 
slipper 
sit tag i os begge. 
Den grå dag 
skal ikke kende 
vore tanker 
og vi 
intet 
forstå. 


                  "),
("Stengrunds tørst",68719476736,"Surrogat


Vi er 
sammen 
på det udvendige 
plan, 
fordi vi ikke 
kan nå hinanden 
indvendigt 


Vi sulter 
efter hinanden 
og spiser 
hinandens 
meninger 
som fattigt 
surrogat 

Vi deler hinanden 
mellem os 
med fælles aktivitet 
som udvendig 
formidler 

Vi lægger sjælene 
om hinanden 
med stadig tommere 
fraser 

Vi fryser 
uden hinanden 
med al den varme, 
vi kan give 
fra os 

Vi kæmper 
mod hinanden 
for at kunne bære
vores smerte 



                  "),
("Stengrunds tørst",274877906944,"Til maleren Erik Heiberg Petersen


Egetræerne

Sommeren
har du ladet
glide
strålende
ned mellem
egetræernes 
kroner.
I blågrønne skygger
ægges
til yndefuld leg
med landskabet
bagved.
Himlen.
Sundet 
og en ildrød bøje.
Uden en tanke
svimlende
lader man sig falde
indad
mod dit
væsens
dybder.

          "),
("Stengrunds tørst",69826773504,"Trolde


Slidte ord. 
Forbrændte 
vendinger. 
Menneskets 
tusinde ansigter. 
Ansigtsrytmik. 
Anstrengte 
livløse 
fraser. 
Vi gør 
som de andre. 
I grå æggeskaller 
klemt inde 
af begær. 
Åbner vi skallerne 
ser vi 
hvinende trolde 
parate til 
angreb. 


                  "),
("Stengrunds tørst",16809984,"Tvivl


Fødes.
Dø.
Leve mere
eller mindre -
kortere
eller længere
ind imellem.
Tror vi noget
er givet
behøver
vi blot
revidere
kendsgerningerne
og uddybe
tvivlen
med et
behersket
hvorfor
et kontrolleret
hvordan
for at stå
tilbage
med et tordnende
Ved ikke!


          "),
("Stilhedens styrke",33587200,"Øjeblik


Ildens flammer
spejler
dine øjnes sorg
Brændte drømmes
længsler
lyser mod dit
ansigt 
Stille
venter håbet
knuget i din hånd


          "),
("Stilhedens styrke",0,"2002-"),
("Stilhedens styrke",33587200,"Afstande


Så langt
til de kolde 
stjerner
Dog længere 
til dig der 
ligger i min 
arm og spejler 
skæret
i våde
øjne


          "),
("Stilhedens styrke",549755846660,"Bambus


Der er stadig
langt fra
bambusens
sang i
vinden til 
styrken af mit 
rastløse jag
Der kommer
en dag hvor mit
hjertes rytme
folder sig ind i
naturens orden
og møder
universet som 
et vindpust gennem 
svajende strå




                  "),
("Stilhedens styrke",549890031620,"Baner


Centre
Periferier
Horisonter for
himmellegemers baner
Beregnede gennembruds
mirakler mødes
af den opgående sols
triumf
Ubesværede åndedræt
indånder livet
grådigt og
lader sig
brænde op
mod den 
fløjelsblå
himmel


          "),
("Stilhedens styrke",698336149509,"December


Solen lavt
i sin bane,
blodrød bag tågen
De nøgne grenes
arrede bark
spændt ud mod
fasanskrig gennem
lysets leg
med skygger
Et vinterhjertes
urolige krav
på et tøbrud
der ikke kommer
Af visne væksters
muldlag spirer
håbets lyse
sommer

                  "),
("Stilhedens styrke",688402759684,"Dom


Jordens uendelige 
flade
Himmelens blankpolerede låg
Fodspor fra intethed til
intethed
Blinde håb 
bærer varsomt smerten 
frem for lyset 
Tomme hænders
råb 


          "),
("Stilhedens styrke",214782083108,"Farver


Sorgens farve
omkring din
ensomhed
Dagligdagens
uforståelige
forudsigelighed
Kørende biler
forbi
menneskeskikkelser
der køber ind
og legende børn
trampende over
forstenede
vækster
Smertens farve
bag en solsort der synger
uset på 
spinkle kviste




                  "),
("Stilhedens styrke",558481047552,"Grobund


Der vokser
solskin i
min tanke
En stædig vished
vil ikke
dø
Der er lys bag
nattens smerte
Der er håb i
spredte frø
der som livets
flaskeposte
stiler mod
en fremmed
kyst
Der er grobund
i det 
hjerte
du har plantet i
mit bryst




          "),
("Stilhedens styrke",8589934628,"Gruppefotografi


Dagene
De talte dages 
kalenderblade
De velordnede 
De planlagt arkiverede
Og dem der blev væk
forlagt og aldrig fundet
Dem der fløj væk
i en storm og
forsvandt til et 
sted hvor de 
lever deres 
eget liv
De lysende dage
De tunge der ikke vil 
gå men bliver 
hængende omkring
Og dagen hvor 
alle livets
dage er indkaldt til 
møde og kommer fra 
nær og fjern
og slutter samværet med 
et gruppefotografi



                  "),
("Stilhedens styrke",687245099044,"Habitter


Salmetunge habitter
i procession
Et fælles fodslags
rytme sliber glatte 
sten under 
ubegribelige buer der 
spænder himlen op i 
tonehvælvet
Ordenes lys i
mørket leger
i blomsterduft
over den hvidmalede
sandhed der en
tid forener 
evighed
og nu

Du
er her
ikke
og hvem er vi
der tror
vi vidste hvem
du var?



                  "),
("Stilhedens styrke",552038563840,"Hengivelse


Hent dagen
af dine hænders
frugt,
af vindens leg i
græsset under det
gamle blommetræ der
bærer snehvide
drømme frem for
lyse nætter
Lad dagens fylde
være dit omfang
og månen lytte til 
dit hjertes sang i
natten



                  "),
("Stilhedens styrke",32768,"Hudløs

Stilhedens magtfulde
billeder,
klangfulde filtre
toner dagen,
nattens dæmoner
mod hjertets lyde
og den aldrig hvilende
længsel,
månelyse tulipan
vuggende mørkets
brise


          "),
("Stilhedens styrke",618945085440,"I mørket


Hold om mig
Lad mig føle jordens 
styrke gennem din 
krop
Lad mig lytte til
evighedens kraft
i dit hjerte
Giv mig den
ro der kan bære
min smerte


                  "),
("Stilhedens styrke",206326235136,"Længsel


Og der
var ikke 
flere dage der
skulle fødes og
ikke flere
nætter der skulle
gå
kun lyden af
et åndedræt med 
universets længsel efter
sammenhæng

Stilhedens lyttende
tyngde


          "),
("Stilhedens styrke",549755813888,"Løftet


De åbne
velsignende arme udstrakt
mod menneskets længsel efter
ord større end flygtige
drømme
Det hængende hoveds
slappe træk
over klippernes
brændende flade
hvor intet slår rod af
sig selv




          "),
("Stilhedens styrke",575660097540,"Løvfald


Hånden
rækker efter
de visne blade
der svæver
afstødte fra
sig selv
i cirkler over
muldens 
flade

Fanger et blad
og lukker sig 
fast

                  "),
("Stilhedens styrke",137472507904,"Måske


Du sagde 
du ville vente
bag lågen
engang

Tvivler jeg
nu på 
dit løfte?


          "),
("Stilhedens styrke",558749450240,"Nyfødt


Den sol der
står op er ikke
den samme som
den der går ned foran
skyggernes langstrakte
rige

Mørkets spor trækker
lange baner gennem
lyse sind under den
tomme himmel 

Nyfødt foran
natten vandrer
lyset dagen 
i møde


                  "),
("Stilhedens styrke",555125047296,"Opbrud


Over 
midnatsskyer
sorgtrompeters hæse 
signal 
i flugt
Rejsen ind
i hjertets længsel
afventer ikke
vantro tvivl
Vinger mod nattens
vinde

                  "),
("Stilhedens styrke",223371853828,"På vej


Det grå lys
har lagt sig
om dig i
et lag der
slører dine
træk

Vore ord
vil ikke
svæve

I dine øjne
ser jeg du
har vendt dig
væk

Du smiler
men er 
borte









          "),
("Stilhedens styrke",206292680708,"Perler


Dine aldres år
løser sig op da
dit udspring
løber sammen
med døden


Tidløse perlers
perlemorsskær 
genspejler
tiden

Dit unge ansigts
lys over den
slidte maske
møder barnets
forventning 
med levede
aldres ro


          "),
("Stilhedens styrke",824767938564,"Proces


Umærkeligt
trækkes 
kraften tilbage til 
skjulte centre
hvorfra livet
fortsætter sin vækst

Visne blade
gribes af vinden
og lejrer sig tæt
i læ

                  "),
("Stilhedens styrke",4,"Punkter


Udsigter
Dalen nedenfor
parat til
dine skridt
Foran dig
vejen
der vil tage
udsynet fra dig
til næste punkt
hvor du
hvilende
kan overse
næste etape



          "),
("Stilhedens styrke",575693651972,"Solhverv


Gennem mælkede
hinder
et solhverv
igen i
sansesløve
funktioners 
isolation
Tiden hældt om i en 
skåret vase
hvorfra håbet
rækker vissent 
mod 
den blege sol

                  "),
("Stilhedens styrke",566935683076,"Stadier


Du var dig
Du var barn
i stakåndet længsel
mod vækst
Livets berusende
styrke 
I den sene dag
ser du dig
tilbage
inden du atter
retter blikke fremad
mod nattens 
første stjerne

          "),
("Stilhedens styrke",549755813888,"Stilheden


Stilheden åbner
sine døre og
lukker op for
skjulte rum
Lader grænser
åbne sig
for mine skridt
der foran mit
mod indtager
dimensioner og 
sletter spor i 
min trygheds
landskab

Frysende fanger
mit blik 
horisonten

          "),
("Stilhedens styrke",77342965764,"Stilhedens styrke


Over alle tage
ser jeg dit 
ansigts lys
hører din stemmes
hvisken gennem 
vindens leg i 
græsset og over 
folkemængders
summen finder jeg
din latter der
ikke mere
søger mig

Den tomme stol
favner 
stillhedens
styrke







                  "),
("Stilhedens styrke",4563402752,"Stjernesang


Stjernerne insisterer ad
kanaler jeg ikke fik 
vogtet mig for i 
løbet af dagen der 
overtog min tanke
og vred den i 
former det smerter at 
rette ud
Glidende drømme lander på 
dugvåde enge hvor 
fortidens vibeskrig trækker 
sølvspor gennem 
tusmørket
Jeg er i mørket under 
stjernerne alene med 
universets stille sang
og uden sorg folder jeg 
mine vinger ud og 
flyver bort i 
evighedens længsel


                  "),
("Stilhedens styrke",549906808832,"Udvalgt

Bærer af livet
Udvalgt som
alt der
lever

Lyset tindrer mod
din pande og
stiller ingen
spørgsmål

Roligt flyder
floden mod sit
udløb i havet



          "),
("Stilhedens styrke",558481154064,"Valmuedrøm 

Åbner sig for 
lyset
Åbner sig for 
vindens leg i 
græsset
De drivende skyer i
livets strøm
gennem
valmuers sarteste
drømme om 
glødende
given sig hen

Åbner sig for 
kærlighedens blomstring
over lerets
kolde maske"),
("Stilhedens styrke",549755813888,"Vinger


Engang var
du fri og
fulgte vejenes
selvfølgelighed mod
dit fængsel

Stivnet i din
kokon forstår du 
ikke de strålende
vinger din
længsel maler på
fuldbyrdelsens
mulighed
          "),
("Tegn",25803358212,"Afstødt

Stuen er tom og det
hvide køkken
badet og det
blinde værelse
entreen som et tilstoppet
afløb der ikke længere
har forbindelse til
trapperne og fliserne
udenfor men hun 
er der som et
aftryk i tiden som et
sæt løsrevne øjne bag et
gardin der blufærdigt
dækker den halve indsigt
som et tilbageholdt
åndedræt bag en 
dør og under avisen et
album med fotografier af
mennesker der ved de
smiler 
lyde som fortabte 
nødråb
raslen af en sommers blade over
våd cement og
hele tiden er hun der
som en bevægelse der
er holdt op med at
gribe efter
tiden som børsten under
spejlet der har gemt et
afstødt hår



November 2005

                  "),
("Tegn",1140851712,"Andensortering

Kan man ikke finde
lykken sammen kan man
komme ganske langt
med at plage livet af
hinanden


Januar 2006
                  "),
("Tegn",283602067456,"Bark

Barken så
nær 
Kronen som et
kontinent
hvor lyset og skyggen
modellerer frie rum
Vinden gennem
sangen over mit
hoved og
visheden om
altings forgængelighed,
om det levende lag der
strømmer fra roden til
den yderste bladrand
uophørligt så længe 
livet vil som
blodet gennem mine
årer 
Barken så nær som
min tanke 
Et insekt på vej
mod lyset i
sangen over mit
hoved



November 2005



                  "),
("Tegn",4362108928,"Blå

Sølvmåne over tåge
Den blå evighed
der forener vore skridt


Januar 2005"),
("Tegn",8589934596,"Bolcher

Måske er det med 
årene som med de
blandede bolcher man
suttede i sig i 
biografens mørke
uden at have  en
chance for at
gemme de bedste
til sidst


November 2005
                  "),
("Tegn",8192,"Broder orm

Hvad nu hvis der
er faldet nogen
linier ud eller
der er foretaget en
redigering fordi den 
menneskelige hjerne har 
nægtet at  
skrive hvad den 
ikke forstod

1. Og menneskets skæbne
skal være knyttet til
ormen

2. Som ormen lever i
jorden skal mennesket
lever over 

3. og den enes liv skal 
afhænge af den andens



November 2005"),
("Tegn",619549032448,"Brødre

De kæmper om
Guds billede og
kalder det sejr når
de vinder


November 2005"),
("Tegn",8590458880,"Budskab

En fugl så
tæt på intet som 
på sig selv,
en spinkel krusning i
tiden
Et vigtigt signal,
kodet i sølv
repeteret  fra 
toppen af
evighed


December 2005

                  "),
("Tegn",135266436,"Cocktail

Dagen er ung
Jeg er gammel

En sprængfarlig
blanding



December 2005
                  "),
("Tegn",824769249280,"Det efterår

Vi ser bladenes triumf mens
livskraften tages fra dem

Vi beundrer deres
skønhed  og ler mod
vindens leg med
deres fald

Vi rasler lag over 
spirende liv og
føler et øjeblik
glædens tillid til
livets forvandling 



Oktober 2005
                  "),
("Tegn",1536,"Disciplin

Øjnene har fattet det og 
ansigtet står ret i 
korrekte  folder
Kun ordene kniber det med
- men munden siger jo
så meget



December 2005"),
("Tegn",549755813888,"Dom

Fuglene ved det og
betaler den fulde
pris som
jordens vækster

Foran den blide 
Dommer ligger 
spørgsmålet som
en spærrende flod

Vi ser den
udstrakte hånd men
kender ikke os selv


December 2005"),
("Tegn",8657043456,"Evigheden

Vi tager mål af
tiden og opgiver at
tælle timer
Vi borer os ind i
nuet og lever
evigt det øjeblik det
tager at flette to
skæbner sammen
Vi er som uden for 
dom

Himlen står på
søjler af lys


November 2005"),
("Tegn",68719585794,"Fare

Længslen efter
harmoni kan dække
over dybe kløfter
Kun den der
føler sig frem
falder ikke i


December 2005"),
("Tegn",134217760,"Fastholdt

Til sidst er der
så lidt tilbage at
der bare er ham selv i
en krop der ikke kan
rumme ham mere
Ord løber tomme 
over magtesløse
hænder

Dråben fanger den
sidste sol og
holder den fast



December 2005"),
("Tegn",68803395584,"Fokus

Jeg er spiddet på
hendes sigtekorn og
hun taler om sin
skæbne
Hvad andet end
tilfældet og min egen
uduelighed anbragte mig her

Som om skæbnen skulle have
øje for mig



December 2005"),
("Tegn",25769837568,"Følsom

Han vil godt
men ikke hvis du 
spørger og
hvis der intet sker
bliver han dårlig



Januar 2006"),
("Tegn",16777216,"Gehør

Dine øjne fortæller at
jeg er en anden og
jeg finder ham
beredvilligt frem

Jeg har ham jo
stadig på
lager


Oktober 2005"),
("Tegn",17332543744,"Gitter

Bag den gyldne 
skræl strømmer duften af en 
dag mellem alle
dage der hører hende til
som huden der
dækker hendes
krop, heden fra den
sortpudsede ovn,
det blanke gitterværk med
fuglen der synger om
forsvundne riger
og tallerkenen med den
splittede frugt i
lidt af det hvide sukker,
omsorgen der
rækker til nye dage men
øjnene finder hun 
aldrig, 
aldrig smilet eller 
stemmen der er borte bag 
tidens grænse som fuglens
stålbundne sang


November 2005




 
 



                  "),
("Tegn",41472,"Graduering

Angsten for 
hvis er 
stærkere end
angsten for
sådan



December 2005"),
("Tegn",25769811970,"Grænse

Vi er som et 
råb i mørket
der søger svar bag
sit eget
ekko


November 2005"),
("Tegn",3072,"Hanegal

Selv den højestgalende
hane er udruget af
samme slags æg som
andre høns -
og inderst inde ved den det!



December 2005
                  "),
("Tegn",68719486976,"Hanekro 

Hvis målet er at
bevare kloden som  et
frodigt livsgrundlag for 
alle dens arter
hvordan sikrer vi så
myggenes trivsel eller
sundheden blandt de
lus vi forsøger at
regulere med alle
midler
Har nogen overvejet
tilværelsen for
Hampagtig Hanekro der
hverken sidder med i
råd og udvalg eller
er sikret adgang til
de elektroniske
medier
Hvad med myrernes
repræsentation i
FN, der hver gang ender i
folkedrab uden
forhandling,
hvepsenes til stadighed afviste
ambassadører eller
de amputerede græsser der
lever som slaver på
vore plæner

Og ikke ét ord om
tallerkener



November 2005"),
("Tegn",32770,"Hunger

Hvad nytter det at
spise hvis sulten
stammer fra hjertet?


Januar 2006"),
("Tegn",8640274432,"I haven

Og det skal være
sådan nu,
ikke at jeg vil det
men mellem hækkenes
hænder er der
alligevel varmere end
udenfor og
hvem har sagt at
udsigten fra de
høje punkter er
mere givende end
inderligheden i detaljen
Mosset gror i skyggen af
barndommen under
hviskende bregner og
ikke så sjældent finder en
levende fugl lidt ro
på spidsen af min
finger
Det bliver ikke på 
nogen anden måde og
om lidt forgylder solen den
hvide lilje der
blomstrer i havens
yderste hjørne



Oktober 2005"),
("Tegn",1207961600,"Invasion

Midt i vinteren står
foråret parat i
kampklædte knopper  
Kulden og mørket
mistolker de
sikre tegn  og
lukker sig om sig selv og
deres undergang
Under døde blade
spirer den spæde
vilje



December 2005"),
("Tegn",0,"Invasion

Som løsladte fugle
forsvinder ordene et
sted bag himlen og
vender tilbage men
bliver ikke genkendt for
ord ligner hinanden
som dråber i regnskyl eller
tagdryp der aldrig holder op
I voksende sværme
slås de om adgang til
yngleplads bag øjne og
ører for at fuldføre
overgrebet på stilhed
der én gang besat
aldrig befris igen



December 2005"),
("Tegn",2,"Kongemarch

Vi er så
fokuserede på
penge og deres
magt at vi 
overser at det væsentlige i
livet er noget man
ofte får for intet eller
allerede har



December 2005"),
("Tegn",41472,"Konsekvens

Jeg er løbet fra mit
ord så tit at det ikke
stiller op når jeg
har brug for løfter og
garantier
Lidt småsnak på
stedet kan det 
strække sig til men 
så snart jeg får brug for
forpligtende ord
bliver jeg målløs og
må gribe til
handling i stedet


Januar 2006
                  "),
("Tegn",8194,"Kur

Det man ikke
vil lære kan
livet bibringe én
ved at ordinere et
par af sine
bitre piller



November 2005"),
("Tegn",69860360192,"Leg

Vi leger med
hinanden 

men leger vi
det samme

eller er det 
ikke leg 

men noget
andet

Det ved vel kun
du og jeg



Oktober 2005"),
("Tegn",4294967296,"Læk

Det drypper fra
tage
Det drypper fra
træer
Det drypper
blide pletter på
mørkets stilhed
Dryp fra den
gamle tagrende der
ikke holder
tæt men
lækker en 
tåre på 
natten



Januar 2006"),
("Tegn",152084480,"Lænker

Der er veje ud men
jeg går ikke og skønt
nøglen sidder i
mine lænker drejer 
jeg den ikke for 
friheden her er
ikke mindre end 
friheden udenfor og 
mine lænker er låst fordi 
det er her jeg 
skal være

Hvor ellers kan jeg
mærke mig selv og
række min hånd til
min skæbne


Januar 2006


                  "),
("Tegn",0,"Læseøvelse

Træet vokser i jorden
Æblet hænger på træet
Kernen  ligger i æblet
æblet plukkes af manden
Manden giver  æblet til konen
æblet spises af konen 
Konen spytter kernen ud
Kernen slår rod i jorden 
Spiren vokser op
Koen æder spiren
Sådan kan det gå


December 2005

                  "),
("Tegn",17180131840,"Løvfald

Da det første 
faldt tog det et
øjeblik
Siden blev det
værre og på et
tidspunkt måtte han 
tage natten til
hjælp, men herregud,
efter en måned var
haven som før, intakt med
sine blade som en
forsikring om at
verden er som
verden skal være



December 2005"),
("Tegn",67108864,"Løvfald

Som på et blad der
er revet løs af
vinden
sidder vi hinanden nær
mens verden farer os
forbi
Og et øjeblik ser du
på mig og
stryger min hånd
Hvad skal vi med
ord så længe vi er
sammen



December 2005"),
("Tegn",10737418240,"Morgendug

Det står skrevet i
duggen

Morgenen er
et blåt gardin
Bag det 
stiger solen og
rydder himlen for
en spejlblank dag
pudset fri for
sorte fingre

Morgenen dufter af
nyslået hø¸ 
høstet i himlen bag
blåt gardin
Hvem vover at
sætte den
første plet på
en nyfødt


December 2005"),
("Tegn",8192,"Målbevidst

Vejen er
hvor mine skridt
vil og
når jeg finder et
mål kalder jeg det
mit og 
begrunder min
rute



November 2005"),
("Tegn",167772192772,"Mønster

Hver dag
inden mørket
kom ud af
jorden,
hver dag 
lige før gik
hun til
vinduet
Hun rejste sig fra
stolen og
fandt vej
Hver dag
inden mørket
kom fandt hun
vejen og
stod foran
lysets undergang som
en der følger en
elsket til
graven 



December 2005"),
("Tegn",550024257536,"Mørket

Så skarpt som
skæret på en
bøddeløkse tegner
mørket på
lyset

vejen mellem
ophøjelse og 
fald


Tilgivelsen 
banker forgæves på 
døren til den der 
har  låst sig inde i 
sit eget sind



November 2005

                  "),
("Tegn",16813056,"Niveau

...som var vi dyr men
vi er jo mennesker eller
er vi bare dyr der
står i lære?


December 2005
                  "),
("Tegn",0,"Notat

Jeg talte 
stjernerne igen
i nat

Der var 
én mere end
sidst


November 2005
                  "),
("Tegn",549755824128,"Nuet 

Korsfæstet mellem
angst og håb til
jordens  tro

Bedende hænder over
spirende muld

Solstrejf på handel med
barnet gave



Oktober 2005"),
("Tegn",8724152324,"Nytår

Som dagene går modnes det i
os at dette må vende i
en kulmination der
gør det af med det  udtjente
Himle skal forgå og nye opstå i
deres sted for at bringe
bevægelsen  frem mod
det mål vi har drømt om og
frygtet fra  første 
begyndelse hvor det
altid var  lyset og 
skyggen,
bevægelsen mod 
den høst vi  har lagt
bag  os uden at
forstå de sikre tegn
Uden kaos ingen fødsel af
kraften der for længst har
forladt os og kun vender
tilbage som et forår der
sprænger den
frosne jord



December 2005"),
("Tegn",8194,"Nåle

Som sommerfugle på
nåle sidder vi fast på en
baggrund der ved hvor
langt vi  flyver


November 2005"),
("Tegn",8589934624,"Nær

Ikke døden men
det ukendte
Ikke udslettelsen men
adskillelsen



November 2005"),
("Tegn",1342177280,"Ophav

Skyggen kravler frem
under træet og rækker
efter lyset den 
ikke kan se som
sin skaber



Oktober 2005"),
("Tegn",1073774592,"Ophedet

Det sidste ord er
endnu ikke sagt men
det ligger i
reden og han
ruger hårdt


Januar 2006"),
("Tegn",68786683908,"Postludium

Til sidst bliver det
omsorgen og
ømheden 

bevidstheden om at
være den andens
daglige brød



Movember 2005"),
("Tegn",11264,"Påstand

Vi stræber efter
tryghed og så snart vi
finder den vender vi os
mod dem der 
ingen har og
kalder dem skyldige


November 2005"),
("Tegn",16820224,"Race

Selvom man ingen
striber har
kan man godt være
en zebra

Som om det par
streger skulle
have betydning



November 2005"),
("Tegn",68786618368,"Regel

Vi ved begge det
er slut, men det 
man ikke siger
gælder ikke


Januar 2006
                  "),
("Tegn",68786618368,"Rod

Min kærlighed er som et
træ der strækker sig ind i
evigheden
Grenene finder mål i
alle retninger og sætter
stadig nye skud
Stammen løfter sig
højt over roden og dybt i
barken har jeg
skåret et hjerte og
et navn


November 2005
                  "),
("Tegn",558345748480,"Rum

Rummets stilhed
går foran og 
åbner mod
evigheden
De sad her før
mig med deres
tillid og lod
hverdagen ligge
inden de rejste sig og
gik med deres svar
Også jeg skal 
gå århundredernes gang
tilbage til nuet der
spejlblankt ligger mellem
mine hænder



Oktober 2005"),
("Tegn",69893883904,"Samvær

Glaspladen er
imellem og
fortætter sig
langsomt til
stål
poleret til et
spejl der har
valgt side

December 2005"),
("Tegn",1048576,"Se

Det kan ske på
en ellers vindstille dag
at en uventet bevægelse
glider gennem luften og
løfter træernes blade og
tillader lyset at 
skrive sine tegn på 
mørket 
som et  kærtegn af en
sommerfugls vinge 



December 2005"),
("Tegn",0,"Skal

Den lammende lyd af
tilbageholdt flugt

En båndlagt viljes
form


Oktober 2005"),
("Tegn",68719576576,"Smil

Jeg leverer et
smil og
pakker det
ind

Jeg binder en
lystig sløjfe

Så kan du sgu selv
pakke det ud!


Oktober 2005
                  "),
("Tegn",8590983168,"Snaps

Jeg plukker træerne
blanke for
solskin og  fylder
alle husets flasker
som jeg propper til og
stiller på rækker i
kælderen

Hele vinternatten kan man
se lyset tegne lange
rektangler  på  mørket
og høre skridt på 
kældertrappen når 
nogen lister ned og
skænker en snaps


December 2005"),
("Tegn",550158729216,"Solfest

Blåvingede skygger på
flugt foran 
kredsløbets veer
Tiden genføder sin
balance over 
vore ofre


December 2005"),
("Tegn",214748364800,"Sorg

Den onde sorg er
sorgen over det
der kunne have
været men aldrig
blev
Sorgen over det
man mistede fordi man
ikke forstod hvad det
var man fik
Sorgen over det man
gik forbi fordi angsten
var større end 
viljen
Den onde sorg
viser kun sit 
mørke ansigt



December 2005"),
("Tegn",549755813888,"Status

Vi har kun vore
nøgne hænder og
det vi kalder tro


November 2005"),
("Tegn",8589934592,"Stilhed

Himlen står på
skakter af lys over
fjorden
Det levende land
tager imod sin
forvandling som
nåde og bærer
livet frem mod
liv
Gyldne penselstrøg
over den 
åbne flade 
tegner den
skabende stilhed


15. november 2005

                  "),
("Tegn",0,"Surt

morgen griber
efter dag men
får ikke fat og
ser den klaske
sammen til 
gråt pjask der
siver ned i 
mørket

Et nul til nat


November 2005
                  "),
("Tegn",8590458880,"Søen

Som ud af et hul i
himlen lyden af
trækkende gæs
usynlig for øjet 
indtil række 
på  række af 
levende fugle opstår af
intet og cirkler over
vandet inden de
vover at dale som
roen der kan komme
til et gammelt menneske
inden hjælpeløsheden sætter
ind
Den blegblå himmel som
blinde øjne der intet ser
men registrerer alt
Som ud af et hul i
himlen en voksende
spiral der afleverer
sit budskab over
søen  i dagens sidste
timer inden natten
henter de sidste konturer af
liv



Oktober 2005
                  "),
("Tegn",558346805248,"Tabt

Han har tabt
humøret 
Han har tabt
potensen og
håret
Han har tabt
potensen og
kampen mod
vægten
Han har tabt
spillet om
guldkalven og
sit ansigt
Han har tabt
troen og kan
ikke finde den
langs byens
gader med
sænket blik

Han har tabt 
sig  men han
leder endnu




November 2005"),
("Tegn",8589967360,"Tegn

Regndråber
morser på
ruden men løber  i
intet før jeg får
afkodet deres sprog
Med spidsen af en
finger tegner jeg min
frustration i
duggen fra min
ånde


Oktober 2005
                  "),
("Tegn",8589934720,"Tema

Vandet løber ud af
hanen et sted i
huset og udenfor på
gaden kører en
bil forbi mellem
andre biler der blander
deres lyde med
tililende lyde i et
tema der
gennemspilles i alle
dagens satser

En smal strimmel himmel er 
solist på
drivende skyer



November 2005"),
("Tegn",8858370180,"Tidens tegn

Tidens tegn er solen
der blodrød står bag
tågen
Tidens tegn er
mørket der strækker
lange vinger ind i
dagen
Længslen efter den
lyse nat der drev
forbi med fugleskrig fra
fjorden
De grønne blades
skygger der som
musikken fra 
krabbe sommerbølger 
skrev sig ind i
sindet i en
endnu ufuldendt symfoni der
venter på sidstesatsens 
kulmination
Forundringen over kroppens
aldring og  mængden af liv
der er forbrugt
Tidens tegn er de 
flammende hyben i hegnet
De fremmede fugle der 
slår sig ned i
mit bryst og tager
hvad de vil 



December 2005"),
("Tegn",18270388224,"To

Hun står bagved 
og hvisker 
Hun står ved
siden og korrigerer
Hun står uden
for tiden og
registrerer 
Hun beklæder
dommerens sæde med
nidkærhed
Hun er der
Kun hjertet går fri 
for hun kender det ikke


December 2005

                  "),
("Tegn",32768,"Travetur

Vinden får tag i
en flig af mit
ansigt og lader det
ikke i fred
I lange flager
forsvinder årenes
bundfald ud over det
åbne hav og 
efterlader et
udkast ingen
skal se
I bilen ligger et
tomt udtryk der
kan gå an hvis jeg
sørger for ikke at
blive set


December 2005"),
("Tegn",8590491648,"Træk

Trækkende gæs mod
en solnedgang der
aldrig hører op

Kaldende lyde over den 
formørkede jord

Himlen mit
savn for nær



November 2005"),
("Tegn",69793251328,"Udfordring

Møghund
Hvad sagde du?
Møghund
Sagde du Møghund?
Møghund
Du skal ikke sige
møghund til mig
Møghund
Nu passer du
på
Møghund
Sig det igen og
det bliver dit
sidste ord
Møghu




November 2005"),
("Tegn",32768,"Udligning

Måske kan vi
mødes et sted
derude
udenfor tiden
nøgne for alder
kun os  to der aldrig
fik brugt vores
chance
Måske kan jeg
endnu nå at
fortælle hvad jeg 
skrev i
mit hjerte og 
måske du omsider
kan åbne dit 



December 2005"),
("Tegn",8859418624,"Udvalgt

Over tagene har
lyset fundet en
sprække i mørket
En solrød forventning
falder ud og lader sig
gribe af et par øjne
Mørkemennesker
langs kolde mure
har andet for i dette
nu der har 
udvalgt sig én



November 2005"),
("Tegn",77310459904,"Vejen vi går

Den første sne falder 
over spor af fødder,
drager  hvide lin over
opkastet jord 
Den første sne er som
forsonende ord der
standser en ond
diskussion og gør det
muligt at se hinanden
som forudsætning
De øde flader
fanger solen i gnistrende
indfald der tænder lys
i mørket og tågen over
intet 
Som glødende eksplosioner
træffer oversete hyben
blikket og indskriver sig i
sindet mellem kræfter der 
aldrig lader sig
se

Og der er endnu tid 
 
Den første sne tager
imod vore skridt og
lader os sætte vort
aftryk som var det
umisteligt vigtigt at
efterlade spor på
vejen vi går



December 2005"),
("Tegn",25769803776,"Vind

Jeg er stormen
der løber panden mod
husene og rasler et
par tagsten så de ved
jeg er der
Jeg er vinden der
flagrer vasketøj og
viser flaget
Jeg er den blide
brise der løber
gennem et 
lokket hår og

om lidt er jeg væk



November 2005"),
("Tegn",558362527744,"Vinger

Vingede spor i
luften over
Babel

Hvad  kender 
sommerfugle til
menneskets liv og dog dør
sommerfugle selvom de har
kendskab til æg og
pupper
Hvad ved sommerfugle om
menneskets rejse fra 
jorden til jorden  om 
mennesket længsel efter 
frihed fra
tyngden,
den skære flugt gennem intet
Hvad ved de om 
begæret der styrter
riger i grus,
om menneskets handel med
guder,
drømmene om
evighed,
den rådvilde gråd i
mørket


November 2005"),
("Tegn",136085504,"Vinterfugl

Sneen dækker
Grenen bærer
Fuglen synger

Sneen dækker haven
Grenen bærer fuglen
Fuglen synger uden at 
åbne næbbet
Jeg lytter

Jeg lytter og 
lader vinterfuglens
stemme ramme et sted i
mit hjerte der er 
lukket for
alle andre end 
den 
                  "),
("Tegn",627123988544,"Åbning

Som et tyst
vingeslag i det
sidste tusmørke kommer
det til mig og 
som et hus der
åbner sine døre og
vinduer for den svalende
vind i den ubærlige
hede lader jeg det
fylde mig til
randen og åbner for
rum jeg havde
glemt
Vi er her så kort men
barrikaderer alle 
indgange for
det  vi  deler med
alt levende som
kunne man 
reservere det der kun 
kan strømme frit

Turde vi i det mindste
holde en dør på
klem




December 2005"),
("Til Dig",68820172800,"Alt og intet

Jeg rækker mine
øjne frem på
åbne håndflader

River brystet op 
og blotter mit 
hjerte mens jeg
råber dit navn

Du vender dig ikke
og ikke et øjeblik 
ser jeg flammen i
dine øjne

To støtter
flankerer 
udkørslen til
vejen der
fører bort
over bakken
                  "),
("Til Dig",68820172800,"Anonym


Jeg åbner 
mit hjertes 
klædeskab. 
Mit inderste 
hjertekammer.
Pakker en 
svulmende 
ladning 
af mit 
overskud 
og sender det 
til dig 
uden afsender 
fordi den direkte 
given sig 
ligger uden for 
mine muligheder 
som jeg må 
vogte for 
dog 
at besidde 
noget 
hvis du skulle 
afslå 
min gave. 
Mit nøgne 
ansigt 
lyser over
vore veje.



                  "),
("Til Dig",68786585600,"At elske


At elske
er at
lade sig
æde op
uden husmoderlig
bekymring om
der er nok
til alle. 


                  "),
("Til Dig",68786618368,"Bøn


Kun ét 
beder jeg 
dig om: 
Leg ikke 
med 
min 
kærlighed, 
for det 
vil dræbe 
- ikke den 
men 
mig.

                  "),
("Til Dig",68786585600,"Du


Fint formet
din hånd 
mod min 
skulder. 

Blidt barnligt 
et ord 
i din 
mund. 

Vi smiler. 


                  "),
("Til Dig",68786585600,"Foråret


Foråret
er en
brandgul krokus
der ædes
af en lysten spurv.
Foråret er en spæd 
anemone
der mases i en
barnehånd
på vejen hjem til mor.
Foråret er også
dig
min skat
når du er
allermest yndig
med gyldne stjerne
i dit brune hår.

                  "),
("Til Dig",73081585664,"I natten


Den nat
vi gik ud
i kulden
var der is
på vejene.
Din kåde latter
jog
i ilende kast
op mod de
tindrende stjerner
dog
lidt efter lidt
blev du tavs
mens den skære stilhed
fik os til
at vokse
ud mod
mørkets kanter.


                  "),
("Til Dig",68786618432,"Låst


Dæmpet 
bærer dine 
skridts lyde 
dig nærmere 
min grænse 
og jeg ved 
du er der. 
Udenfor. 
Altid udenfor. 
Hvorfor 
åbner jeg dig 
ikke ind 
hvor jeg tørster efter 
dig? 
Hvor jeg tørster
efter 
din lyd 
din duft 
dine farver.
Din krops 
tyngde. 
Varmen. 
Din hud mod mine 
hænder. 
Hvorfor 
åbner jeg ikke 
min verden 
mod dit væsens 
given 
sig hen? 
Ved jeg ikke 
hvordan 
man tager imod 
eller vil jeg 
ikke risikere 
den sårbarhed 
det koster 
at tage imod 
en skænket 
nærhed.


                  "),
("Til Dig",68786618368,"Molen


Bølgeslag
mod molen
Skumsprøjt
Min tomme
hånd
Og din
Regnbuens
segl i
dit øje"),
("Til Dig",68786585600,"Nul kredit


Du får ikke mere nu. 
Jeg har sat pris på 
min kærlighed. 
Resten er din omgang. 
Jeg har været 
som den vinduesløse, 
der hovedløst 
bar solskin 
ind i mørket. 
Et ordentligt hus 
skal have vinduer.


                  "),
("Til Dig",87056973824,"Offeret


Alt hvad jeg
ejer har jeg
pakket sammen i
ét blik som jeg
planter for
dine fødder

Du ænser det
ikke før mit
offer har
omslynget dig tæt i
et ubrydeligt
greb
                  "),
("Til Dig",68787109888,"Ophold


Trætop 
Fugl 
Klapren fra flygtende vinger 
og du min elskede. 
Det sner 
Tøsjap 
der rammer den 
dyngvåde jord 
og bliver borte.


                  "),
("Til Dig",69324537856,"Parløb


Du løber lidt foran mig,
men vi følges ad
på de sædvanlige 5 km
gennem morgenduggen.
Landskabet skønhed
og vores småsnak
blander sig til en rolig meditation.
Din slanke skikkelse
lidt foran men 
vi følges ad og først
den sidste km
følger du din krops lyst
og strækker ud.
Du er hjemme et minut før mig
og da jeg åbner døren til badeværelset
står du allerede under bruseren
og lader vandet plaske ned over
din dampende krop.
Hurtigt får jeg tøjet af
og trykker mig ind til dig.
Lægger hovedet til dit hårede bryst
og griner.
Vi klarede den.
Skal vi tage 20 år til?



                  "),
("Til Dig",68786585600,"Snore

Snore af sølv holder
månen fast over
vore ord der
får mod til at
svæve væk gennem
mørket 
Stilheden væver med
guldsnor i nat  og
spænder bro over
kløfter



                  "),
("Til Dig",68790910992,"Steppebrand


Sorte stammer 
røde valmuer 
foran. 
Som ild mod 
øjet. 
En steppebrand mod dit bryst.


                  "),
("Til Dig",77376520193,"Til dig


Lad mig sige 
det 
sådan 
fordi 
det er den måde 
jeg kan beskrive 
min virkelighed på 
lige nu: 
Er verden 
et beklumret 
kælderrum 
med ét 
lufthul 
hvorigennem 
luften 
og 
lyset 
fra 
Guds morgenfriske 
verden 
strømmer 
levende 
ind, 
er det 
lufthul 
som 
du 
for 
mig.


                  "),
("Til Dig",84967424,"To hjerter

Da du gav mig dit
hjerte
rakte du mig mit, du
havde fundet
et sted det ikke
hørte til

Da du gav mig dit
hjerte
gav du mig
mig selv tilbage og
gjorde mig hel



September 2007"),
("Til Dig",69860327424,"Tosomhed


Bag avisens 
barrikader 
drømmer jeg 
om et liv 
a-l-e-n-e! 
Under mig 
memorerer gulvtæppet 
dine seneste 
uforskammetheder. 
Dit lig 
hænger skændet 
fra lysekronen. 
Dit blod 
driver fra gardinerne 
og fra væggen 
smiler du 
posthumt 
ud over 
vort værk: 
Det fælles hjem.


                  "),
("Til Dig",68786585600,"Ved vinduet


Jeg lytter.
Du kommer
hen imod
mig
bagfra
på nøgne
fødder.
Min ryg
rækker armene
ud mod dig
og smiler
da du blidt
trykker dig
ind mod
mig.
Under min arm
kigger du ud
på marken.


                  "),
("Til Dig",68786585600,"Vi


Min kind 
mod din. 
Min hånd 
bag din 
nakke 
Lidt senere 
dine læber 
mod mine. 
Dit sind 
i mit 
og verden 
ånder 
for os 
alene. 

                  "),
("Til Dig",68786585600,"Vore hænders spor

Vore spor over
den hvide
flade
altid tæt

Nu ser vi dagen 
svinde

Vore hænder sætter
ingen spor
                  "),
("Undervejs",17179869184,"Åbenbarersken

Bag døren 
tager hun stilheden på og
træder ind foran
Med spatler, pensler og
nøgne hænder
trænger hun gennem
hinden og åbenbarer 
sandheden bag
Man kalder hende
kunstner



29. juni 2004"),
("Undervejs",549755813888,"Øjeblik

Skyerne skilles  og et 
øje kigger ned og 
blinker inden
forhænget trækkes for
igen



29. maj 2004"),
("Undervejs",17179870208,"Øvelse gør mester

Hendes konversation er
overvældende men
hun træner også
en del


Juli 2004"),
("Undervejs",98564,"Afmagt

De små kan
gøre ens liv
besværligt
Senere får de
bedre fat og
knuser ens
hjerte med
vægten af et
smil 


Juli 2004"),
("Undervejs",0,"Ansat

Og se om jeg ikke 
spyttede stenen ud 
uden at tænke over det
der er  meningen med 
det hele

Jeg fyrede den lille
rumkapsel af uden
hensyn til livet
der ånder bag de
spinkle vægge

Jeg må have én til
og føler mig med ét
ansat i en højere magts
tjeneste 
Der vil unægteligt gå en 
del tid med at så på de 
rigtige steder og
der følger vel en 
slags guddommeligt ansvar med
sådan at tage aktivt del i 
kunsten at række livet
videre for det vil jo 
uvægerligt føre til 
store områder med 
kæmpende vækster der
hvis alt går vel vil 
stå og byde sig til med
saftigt struttende frugter

Det går vel at tage en enkelt til?





August 2004"),
("Undervejs",40962,"Antenne

Glæde kan du
forbruge og
vælte dig i uden
skam for
jo mere du 
trækker ned jo
mere er der til
alle



30. juni 2004"),
("Undervejs",1048576,"Appetit

Denne dag skal blive
min
Jeg vrister den
løs og æder den som 
et saftigt æble



26. juni 2004"),
("Undervejs",8198,"Børn

De første tredive
fyrre år forsøger vi at
blive voksne
Så følger en 
kort periode hvor vi 
benægter fakta indtil 
vi er blevet så gamle at
vi opgiver ævret og
går ind i de børn vi
har været hele tiden



22. juni 2004"),
("Undervejs",0,"Bag bjergene

Bag bjergene ligger
andre bjerge
Her vokser en
lilje der snart vil
springe i blomst


9. juni 2004"),
("Undervejs",2,"Beslutsomhed

Jeg vil og
jeg vil ikke så
jeg bliver og
lader viljen
holde fri


21. juni 2004"),
("Undervejs",618475298818,"Blændet

Er man blændet af
Guds kærlighed overser
man let lidelsen i sin
skygge



27. juni 2004"),
("Undervejs",8589967360,"Bladet

Hvert år udvalgt hun et
blad på det store
egetræ og gjorde det til
sit 
Hun besøgte det gennem
sæsonen og nogle
år fandt hun det på
jorden når sommeren
var forbi mærket med et
glødende hjerte



25. juni 2004"),
("Undervejs",42074112,"Blå

Den blå
himmel den blå 
dag
Gylden evighed
trækker lange
skygger af 
murene 
fanger gadens
skarpe kanter og
bygger rum 
Bronzelemmer
og lysende
øjne 
Stemmer der
taler til nogen de
kender i
sønderklippede
ord og lyden
af fødder der
går af sted og
forbi



August 2004"),
("Undervejs",68736352256,"Bolcheposen

Han følte sig som en
bolchepose alt
for mange fingre
havde krammet for at
få indholdet ud
Rigdom eller
afmagt -
han vidste det
ikke men nu var
posen i hvert fald
tom


12. juni 2004"),
("Undervejs",8192,"Cyklus

Sommerens blomstring 
skjuler forårets 
spild og  sigter efter 
solen for at 
skyde over 
vinterdøden


August 2004

                  "),
("Undervejs",41943040,"Den lyse nat

Sommernattens
perlelys græder mod
ruden der
blindt lader smerten
falde ind 

Perlelatter fra
gaden der driver
kåd forventning
forbi


7. juni 2004"),
("Undervejs",50366464,"Drømme

Han åbner sit hjerte og 
nogen går
ind

Herrefolkets drømme 
legitimerer støvletramp

Gudshuse styrter til
grå ro og han
lukker sig om
det der er
endnu



30. juni 2004"),
("Undervejs",0,"Drømmen

Vi rejser os med
drømmen og
lader den 
gå foran ind
i den
formbare dag


1. juni 2004"),
("Undervejs",77376782336,"Efterår

Bladene rasler i
vinden
skraber lag af
tiden og
finder dit
ansigt  bag
min erindring
Dine øjne fanger
mine og
spejler smerten 
Du tænder dit
smil og sender
bladene dansende ned
ad gaden


August 2004"),
("Undervejs",16777216,"En vasker

Under dække af
en dårlig undskyldning
forsøger jeg at
slå paraplyen ned men
får straks en
skylle i hovedet og
slår kontra
Det højere diplomati er 
vist ikke lige 
mig

August 2004"),
("Undervejs",0,"Enerne

De dage der kun
vil som jeg 
går i flok med 
dem der
stritter imod
men der er dage der
går for sig selv og
sætter kursen for
alle uden at
spørge om lov



11. juni 2004"),
("Undervejs",8589942784,"Ensrettet

De vender ikke
om og
du kan ikke
finde stederne
igen 
Du kan ikke
løfte dig ud
af tiden eller
gemme dig for
livet der kun 
vokser frem og
aldrig tilbage


18. juni 2004"),
("Undervejs",32770,"Erfaring

Det nytter ikke at
lede i hjernen, hvis
det man søger bor i
hjertet


August 2004
                  "),
("Undervejs",68719509536,"Evig

Den vilde
blomstring vil
fortsætte når vi
ikke længere
kan se den 
skære blå eller
høre lyden af 
vinden der
hvisker mellem
strå 
Et solstrejf over
visne blade i en
uopdaget stund bag 
tid der aldrig kan
spoles tilbage til
din hånd der
strejfer min et
splitsekund af 
evighed








26. maj 2004"),
("Undervejs",68787765248,"Farvelade

Himlen er
blå
Havet 
efteraber
Skoven bag os
stjæler farven i
dit øje
Intet er så
rødt som
flammen i mit
hjerte


2. juni 2004"),
("Undervejs",2129920,"Forlis

Natten tog sit og
da lyset kom stod
skibet fast 

Trådene holder ved og
forbinder
rækker bagud og søger
frem mod nye
vedhæftningspunkter

Noget brister og 
ændrer balancen 
Alt slides løs og
sendes i drift

Den faste grund
giver ikke
slip og
ruster sig mod
den næste
bølge




30. juni 2004"),
("Undervejs",549755854850,"Fremad

Man skal
godtage sig selv
- ikke sine
handlinger
Dem skal man
tilgive


August 2004"),
("Undervejs",1073752576,"Frygt ikke

Alle skyggers
øjne

Dunkle
områders 
ører

Alle steders  lydløse  
mekanik
observerende 
livets ytring 

Rum vogtet af
maskinel
huskemagt

Alt er roligt i
dag så hvorfor er
børnene bange



August 2004"),
("Undervejs",549756870690,"Gaven

Kun ved at
leve hver dag som om
livet var
evigt kan man
forberede sig til
at dø og kun ved at
leve med døden hver 
dag kan man modtage
livet som var det en
gave


4. juni 2004"),
("Undervejs",33589280,"Gidsel

Revet ud af din
angst ser du den
fælles skyld stå
vagt
Kløften kan kun
fyldes af
kærlighed

Du skal dø for
kulturens mærke og
ser lyset leve på
muren overfor



30. juni 2004"),
("Undervejs",17179870240,"Gravskrift

Du tog dig altid
godt betalt 
men du fik også kun
hvad enhver kan få
så længe pungen er
fuld af 
penge


August 2004"),
("Undervejs",8192,"Håb

Der kommer en
tid hvor vi kan
kigge frem uden
samtidig at se os
tilbage -
hvis vi tør


Juli 2004"),
("Undervejs",66818,"Handicap

Det er tungt at
være barn hvor
mor har sat sig på
al barndom


15. juni 2004"),
("Undervejs",68786585600,"Heks

Sortere end natten
dine øjnes dyb

Styrtet ud af din
hånd til brøndens 
uafvendelighed

Mellem slanger
søger jeg
Heksens gave


23. juni 2004"),
("Undervejs",8606711808,"Helle

Dagen løber foran
mine ønsker og
tager først

men i 
hegnet står en
blomst og skaber
rum 

der gemmer jeg
mine drømme


August 2004"),
("Undervejs",68786585602,"Hvad ellers

Kan du ikke
holde af
kan du vel i 
det mindste
holde afstand 


30. juni 2004"),
("Undervejs",16785410,"Hvem der?

Din krop er
et dyr
Hvad med
dig?


5. juni 2004"),
("Undervejs",549756870656,"Jagten

Jeg har ledt
overalt men 
kan ikke finde den
Jeg har være alle
steder både
indenfor og udenfor
lands lov og ret
Den er der ikke men
mange steder har jeg
set mennesker gå rundt med
stille smil
Måske kan 
man slet ikke 
finde den men
må vente på at 
blive fundet af
den



3. juni 2004"),
("Undervejs",2,"Jamen så hold dog ...

Der skal mere til at
holde en tale end at 
sige nogle ord


30. maj 2004"),
("Undervejs",77377568768,"Jordbær

De jordbær var
søde og gemte
mere end sommerens
drømme
Jeg ser  gennem
glasset du sidder på
hovedet og
finder det
passende at
drikke din
skål


August 2004"),
("Undervejs",549755813888,"Kære Gud

Jeg beder dig
se efter om du har
min pinkode
liggende for 
jeg har den ikke
Jeg har ventet længe og
set hvordan andre har
fået succes i  ung
alder mens intet er
lykkedes for mig 
Har du glemt mig
Gud
Husk mig  inden
det bliver for sent
Send mig koden så
hurtigt du kan for
jeg kan ikke længere
skjule at noget er 
galt
Tving mig ikke til
desperat handling
Der er jo det andet 
sted selvom jeg mener at
vide at din
efterlivsordning er
langt den behageligste så
jeg venter en tid 
endnu
Jeg råber så højt jeg
kan: Glem mig ikke 
Tænk om det 
rygtes at 
en af dine er i 
nød



Juli 2004"),
("Undervejs",0,"Klokke

En klokke
ringer
Hvem ringer klokken 
for?
Det er let at
lade som 
intet
Lad klokkerne
klokke for
åbne ører
Lad klokkerne
vække de
frelste
Jeg ved hvad
klokken er
slået og der er
altså noget
som klokker ikke
ved



August 2004




          "),
("Undervejs",68786585600,"Kom

Hvorfor bruge
mange ord 
Kom
lad os klippe hul i
tiden og tage en
elevator op i
i Herrens lomme
Vugge lidt i
evigheden uden
ord
Uden ord



17. juni 2004"),
("Undervejs",16777218,"Komet

Vil du arbejde med din 
fremtid 
må du begynde med dit
spor


19. juni 2004"),
("Undervejs",618475300864,"Kun

Der er kun de
dage der er
dine og deres
Der er kun
den høst der
modnes af
sæd 

Der er dine
hænder og 
deres 

Vi var 
Vi er,  
kalder dagene
vore og 
indsamler
høsten hvor
nogen sår


30. maj 2004


                  "),
("Undervejs",68786585600,"Kære ven

Kan du huske den
aften vi
mødtes for første
gang
Du stod  på broen og
jeg kom gående
Du så på mig med en
fremmeds blik men
du vidste jo 
heller ikke det
var mig eller
gjorde du
Vi gik i
hvert fald over
broen sammen og
blev til os


August 2004


                  "),
("Undervejs",2056,"Lille Guds barn

Lille Guds barn 
hvem
skader dig og
slukker lyset i
dine øjne

Lille Guds barn
hvis hænder er
det 
der plukker træerne
rene for dine
drømme

Lille Guds barn  
din tomme hånd
bliver overset og
ingen hører 
gråden der 
striber den
blanke himmel


August 2004"),
("Undervejs",68820140032,"Lysår

Mellemrummene 
strækker
sig
Før hentede vi
varme hos
hinanden men
afstanden er
blevet for stor og
de landkort vi
tegnede sammen
kan ikke bruges her
hvor vejene kun 
fører væk fra de 
mål vi satte os 
engang 


30. maj 2004"),
("Undervejs",268435456,"Mønster 


                          Så langt der
                          end er mellem

mørkets stjerner




                           danner de dog

et mønster at
fortøje ved


                    Et posthus for
                    tanker uden
                    adresse



                         En knage for
                         uforløste behov



               En målskive 
               dirrende løfter

i mørket


                           Et sikkerhedsnet






                              mod
                                ødet   


16. juni 2004"),
("Undervejs",16777216,"Maraton

Der er kun mig og
sandheden

Der er kun
mig


August 2004"),
("Undervejs",549755822082,"Mening

Måske er det
sådan at
hele livets
mening kan skænkes i
ét sekund
Måske er det
sådan at den kan
mistes i et
andet


Juli 2004"),
("Undervejs",68787634176,"Måske mere

Mand i den
tredje alder
søger én til en
fælles fremtid -
måske mere,
har et 
goody humør og
alle får svar
Ses


August 2004"),
("Undervejs",16777218,"Nå dér

Kan man ikke 
finde sig selv er
det lettest at 
lede der hvor 
man er og se om 
man passer til den
nærmeste plads der
står tom


27. maj 2004"),
("Undervejs",69810036738,"Nivellering

Man bliver svimmel af
at kigge ned i sit
afmagtshul så det
gør underværker at
få det fyldt det op med
andres svage punkter


6. juni 2004"),
("Undervejs",269484032,"Nu

Lysende øer af
sommer
fanger min 
hånd og
lader den 
danse under det 
grønne tag

Dagen er lang mod
mørket der lader sig
tæmme af 
øjeblikkets sang


August 2004"),
("Undervejs",0,"Over under

Det er udenpå og
det er
indeni
Det er
nede under
overfladen og
udenpå som spor
efterladt af en snu
vilje der uvægerligt
forregner sig og
åbner uønskede
indblik
Det er
masker over
afslørende træk
Parykker på
skaldede hoveder og
latter som is over
gråd
Det er 
forseret tale spændt ud over 
slugter
Afværgende rapporter med
skjulte fejl
Det er sandheden og
løgnen der klynger sig til
hinanden i en tæt dans over
skåret glas





28. juni 2004"),
("Undervejs",85899345920,"Q32

Hun er 
sygeplejerske
32/176/65 
Håret langt og 
bølgende lyst over
brysternes toppede 
c-skål
Hun er fræk og
ved hvad hun 
vil
stikker hanner i
brand og varmer sig ved
deres flammer
Tit er hun gift og
manden af sted og hvem
skal så spille på
bordet
Og hun vil ikke ringe for
havfruer kan ikke
gå og fatamorganaer har 
ingen stemme


August 2004"),
("Undervejs",16785408,"Redaktionen

Bag vore handlinger
ligger motiverne i
flere lag uden på
hinanden
De inderste så
dybt at de færreste
kender til dem 
Bag alle lagene sidder
den nøgne vilje og
klipper sandheden til så
den tåler at blive set af de
redaktører der færdiggør de
synlige detaljer




24. juni 2004"),
("Undervejs",2050,"Regler

Reglerne 
skal kun styre 
os 
fordi vi styrer 
dem


Juli 2004"),
("Undervejs",16777216,"Rutine

Han lægger frakken og
stoltheden
Habitten og sin
griskhed
Fremmedgjortheden ryger af
med skjorten der
først er rystet ren for
misundelse
Han hænger
frådseriet op og
lægger sin liderlighed på
køl ved siden af
vreden
Løfter sine vinger og
flyver ud med
lyset


30. juni 2004



          "),
("Undervejs",8590983168,"Samlerne

De går ud over
markerne og samler
stilhedens puslen af
dens eget rum
Forvarer dens
frugtbare tyngde og
bærer den
med sig som et
løfte til dagen

Den ligger i
deres øjne som
rester af et
smil



13. juni 2004"),
("Undervejs",688268519424,"Sammenhæng

Selv dagen
græder i dag
Hvordan kan den
overhovedet
stråle og rulle sig
ud for enhver der
vil bruge den  til
sit  uden  at se den
levende sammenhæng
der følger med at
være en af
alle
Når dagen græder er
den grå og skænker
stadig sine gaver 


August 2004

                  "),
("Undervejs",549755822080,"Skabelse

Universet skarpt
tegnet på
skabelsens øjeblik der 
evigt ånder
livet gennem sin
skabning


Juli 2004"),
("Undervejs",16777218,"Skaberi

Er man træt af at
være den man er
må man forsøge at
skabe sig en anden 


17. maj 2004"),
("Undervejs",32768,"Smaragd

Som et glasskår  mellem
havvåde sten skyller
mindet på plads bag
brændingen

En tilslebet trussel på
stranden
En tæmmet vildskab i
din hånd



20. juni 2004"),
("Undervejs",206426865664,"Steril

Og jeg skulle vel
ikke gøre andet end at 
række hånden ud og
nå frem gennem
mørket
men jeg gjorde
intet andet end at
lytte til den
stumme bøn 



Juli 2004"),
("Undervejs",77444677632,"Stjernesamlerne

De smelter ind i
natten og lader sig
blæse med 
jordens dufte ud
over det åbne
hav  hvor de samler
hjerterne fulde af de
faldende stjerner på
spidsen af 
evigheden

Over tidevandet går de i
land med mørket
svøbt tæt om 
deres ulmende
byrde


August 2004"),
("Undervejs",68719476736,"Sturt nummer

Hvem tror vel på
løfter svøbt i
måneskin
Det skulle da 
være én der
tager måner
alvorligt og
sulter efter drømme der
snor sig opad mod
lyset som 
cirkusprinsessers
spindelvæv
Det kolde gys
står parat i
manegen og
optræder uden
sikkerhedsnet i nat


August 2004


                  "),
("Undervejs",2050,"Så enkelt

Vi 
får fred den
dag, det er fred
vi 
vil have



30. juni 2004"),
("Undervejs",2050,"Så enkelt

Vi 
får fred den
dag, det er fred
vi 
vil have



Juli 2004"),
("Undervejs",4,"Tankevækkende

Før var det de
gamle der døde og 
døden var ikke
noget man
tænkte på
Nu er der
et pænt antal
tomme pladser efter
nogen fra ens
egen tid og
hver morgen kan man
læse små annoncer om
flere der er
sovet stille ind
er gået bort eller
har fundet ro




30. juni 2004"),
("Undervejs",549755813888,"Tavlen

Tavlen var ikke af
træ og skriften
formet med
kridt
Så gik hun ned ad
bjerget på
randen af den
nye dag


Juli 2004

                  "),
("Undervejs",32770,"Tilsmagning

Ingen kan fjerne din
bitterhed men er du
heldig kan du
tynde den op med
udsyn til den
antager karakter af
personligt raffinement


8. juni 2004"),
("Undervejs",94556389412,"To

De vil så gerne
følges ad de
to og tør ikke
vente
Sammen henter de
tæppet og plukker sommerens
sidste rose 
Så åbner de garagen og
lukker for
sidste gang

Så langt er de
gået hånd i
hånd og klarer vel
også den
smule vej


Juni 2004"),
("Undervejs",17179870208,"Ulideligt

Han bredte armene 
ud lige så langt som
sit smil og fortalte igen
hvad alle jo vidste -
at det var ham 
der sad på 
de vises sten


10. juni 2004"),
("Undervejs",67108864,"Under dit
blik

En mur af
stilhed i
støjen
Et lydløst
rum i den
kogende strøm
Præstindens øjne
spejler lyset bag 
opalen
Det næste træk er
dit


14. juni 2004"),
("Undervejs",137438953472,"Vækst

Hun er udset til at
bære den tungest
sorg og sætter den et
øjeblik ved 
medfølelsen inden hun 
tager den op og 
løfter den ind i
sin stilhed 
igen


17. maj 2004"),
("Undervejs",0,"Ve du?

Ve du mæ u å
lai så kom
Vedu mæ  uå
lai såkom
Vedu mæuålai zkm
Vedumyuålai
Så Kom!


August 2004"),
("Undervejs",549773647872,"Vished

Med ét vidste hun at
hun ikke blot levede for
sig selv eller
menneskeracen
Uden tanke på
Gud eller Djævel
gik hun ud i dagen og
plukkede en  buket
meningsfuldhed som
hun satte midt på
bordet i spisestuen og
da hun havde tændt 
lyset trak hun ikke
gardinerne for men lod
frimodig mørket
kigge ind


Juli 2004"),
("Undervejs",549890072576,"Ødeland

Uanset hvad vi
gør
bruger vi vores 
tid sekund for
sekund
Om et minut er
der et minut
mindre at gøre
godt med
Om en time en
time
Om et år et
år
Det nytter ikke at
være nærig med sin
tid 
Så er der måske
mere fremtid i at
være ødsel

August 2004

                  "),
("Vækst",283467841536,"Arkitektur


Træer 
står 
kun 
på 
række 
af 
pligt. 
Fordømtes
grønne 
kolonner 
lever 
betonblokkes 
drøm 
om 
natur. 
Placeret 
Behandlet 
Beskåret


                  "),
("Vækst",0,"Bevægelse


Vigtigt er
at kende tiden.

Vide hvornår
du skal gå.

Ikke som blade
fanget af blæsten
lade dig
føre i viljeløst
bånd.

Gå før
angsten for at
miste
gør dig
til
halvhjertethedens
udslugte
fange.

          "),
("Vækst",0,"Forstår du?


Al verdens skønhed
er samlet
i et ord.
Et eneste
simpelt
dagligdags ord
du har sagt
så mange gange
men glemt igen.
Den dag
jeg finder
det ord
holder jeg op
med
at tale.

          "),
("Vækst",8606711808,"Fundamenter


Svaret på
alle
dine spørgsmål
findes
godt nok,
men ikke
der,
hvor du søger
dem.

Hvad hjælper
det at spørge
mod vinden.

Svaret på
alle dine
spørgsmål
ligger
lige for,
under dig,
men dækket
af hele dit
livs
muldlag,
som du kalder
slagger
men
som er
muld,
der lever
dig
lige nu
og nedenunder
ligger
dine fundamenter,
der afstiver
hele dit væsens
omfang
og viser dig
vildnissets
veje.


          "),
("Vækst",0,"Inspiration


Der er hul
ind i
evigheden
og et sært budskab
finder vej
til et sind
der måske
intet forstår
men føler sig presset til at
række 
videre
i et udtryk
andre må folde ud.

Forpinte uklare 
øjeblikke
der sætter din handling i stå.



          "),
("Vækst",17196646400,"Mig


Der er 
den lille fede, 
grinende budda 
i maven 

Utøjlelig 

Der er 
den gamle grå dødning 
bagved 
og 
et eller andet 
sted 
i sit eget rum 
den lille lyshårede 
dreng 
det er synd for 

Hvor er den voksne 
ansvarlige 
mand? 

Er det mig?
                  "),
("Vækst",16777216,"Puslespil

Angsten
for at skulle
leve
den falske lykke
krydser
min vej.
Jeg vil være
uerstattelig
og hel.

Jeg samler
glasskår
af det
jeg har smadret."),
("Vækst",0,"Smerte

Angsten 
for smerten 
er ofte større 
end smerten selv
Dog gives der 
smerter 
selv angsten 
ikke fik sporet 
dybden af



          "),
("Vejen",2,"Øgler

De forventning der
brækker vingerne ved at
flyve imod
virkeligheden 
forvandler sig let til 
hvæsende øgler



8. marts 2004"),
("Vejen",17179869184,"Økonomi

Han fik alt og 
til den rigtige
pris
undtaget livet 
der gik mens 
han sad og
regnede den ud


6. april 2004"),
("Vejen",17858560,"Afmagt

Som det er kan jeg
intet gøre 
min indsats må
være en
anden 
måske bare den at 
bære glæden over 
livet der stædigt vil at 
jeg er til


3. april 2004
          "),
("Vejen",17314087936,"Bla bla

Hun sender effektivt
men modtager så
dårligt at hun i
realiteten er 
samtaledøv


4. februar 2004"),
("Vejen",549755822082,"Blokeret

Så længe man tror at
tilgivelse er en ret 
har man for fulde hænder til
at tage imod


9. april 2004"),
("Vejen",68786585600,"Broen

Det var ikke fordi jeg
lærte dig at kende
for jeg genkendte
dig straks
Det var ikke kærlighed ved
første blik men en
genforening af  to der
så hinanden for
første gang


18. marts 2004


                  "),
("Vejen",16777216,"Bøn

Lad mig se verden som
den er når den ikke er
forvansket af mine 
filtre
Lad mig se med 
fluens facetterede blik og
smage på livet med 
alle fluesanser 
Være stum i
mørke huler
Glide som
en skygge over 
marker 
Følge lyset til
havets bund og 
bide kulden
under en stjerne på 
kanten af et 
fjeld
Lad mig være en 
slange under
ørkenens dæmon 
Sluge byttet på grænsen til
døden og ånde min 
duft i takt med en 
viol halvvejs gemt
under et hegn i marts
Og lad mig sanse med 
kloden som den svinger sig om 
solen i sin bane mod
sig selv


13. marts 2004


                  "),
("Vejen",16777728,"Cirkelen

Den er lille i
dag
Lægger sig tæt til
kroppen og
holder fast
Andre dage er
der mere plads men
du har aldrig meget
albuerum og må altid 
tage forbehold når du
planlægger din
aktivitet


22. februar 2004

                  "),
("Vejen",4563436032,"Dæmoner

De har fundet mig og
vil have det sjovt så
selvom jeg ikke vil
lege kan jeg lige så
godt stå op og 
forsøge at låse for 
det der gør ondt hvis det 
slipper min ro og følger
dem ud på nattens 
formørkede veje


15. marts 2004
                  "),
("Vejen",50364992,"Døren

Ingen afstand større end 
tykkelsen af en 
dør der har
smidt nøglen væk og
rustet sine
hængsler 
Ingen smerte så tæt som
frosset længsel der
ikke vil 
tøs op


17. marts 2004"),
("Vejen",69810003968,"De ved

De ved det og
har redegjort for det i
alle slags
skrifter
De ved det og
mangler kun sjældent
ord
men dín sandhed
er kun skrevet ét sted og
kan kun læses af én der 
ved hvordan man staver til
sandhed med uplettet  
levende 
kærlighed


20. april 2004



                  "),
("Vejen",68786618368,"Decoupage

Med den lille
spidse klippede hun
ham ud og
lagde ham væk i
en skuffe
Skar ham af og
låste ham inde
På hans plads
limede hun en
tvilling, kyssede de
to søstre og smed
nøglen væk


6. marts 2004

                  "),
("Vejen",68719476736,"Delagtiggjort

Du har lagt
ring om,
beslaglagt og
nu sætter du
angrebet ind på 
én der bare
kom forbi


21. april 2004"),
("Vejen",16809984,"Den

Den lagde sig om ham og
trængte igennem hans
celler
Den kuplede ham op i
is og gjorde ham til et
håndterbart objekt
Den greb hans sjæl og
plantede den ud på
toppen af et bjerg som 
ingen kunne nå
I natten sprang den ud
og lyste som en stjerne der
langsomt blev slukket af
dagens grå



7. marts 2004"),
("Vejen",68719476736,"Dommen

Bag dine
korsfæstende ord
ser jeg dit
ansigt og ved jeg 
er fri


28. februar 2004
          "),
("Vejen",268443648,"Dryp

Lyset tindrer
dråbens beslutsomhed og
lader sig falde ind i
mørket



10. marts 2004
                  "),
("Vejen",2050,"Ensretning

Man kan godt få
mange viljer til at
rette ind efter en
men først når 
alle viljer af natur 
strømmer i 
samme retning
bliver der
fred


19. april 2004"),
("Vejen",8606712320,"Erkendelse

Undervejs gik det op for ham at 
han ikke vidste hvor han
skulle hen og i hvilket 
ærinde han var ude
Ved nærmere eftertanke
stod det ham klart at 
han ikke anede hvem han 
var eller hvor han 
kom fra
Da den første fodgænger
gik lige igennem var det et 
chok men selv den slags bliver
hurtigt en vane


12. marts 2004
                  "),
("Vejen",1056768,"Evighed

Der er altid en
chance bag
horisonten 
bag tapetet
i et tilfældigt
smil
Der er altid en 
ny vej et sted på
ruten der fører til
stedet hvor
kortene blandes  og
deles om igen indtil det 
evige øjeblik hvor alt
løber sammen i 
fælles vilje der ikke 
kender til
mulighed


2. april 2004"),
("Vejen",206158430240,"Faciliteter

Det er nyt
velindrettet og
funktionelt
Der er det
hele men
latteren flyttede
ikke med
Den nære
fortrolighed  gennem 
et langt liv eller
skridtene fra
den anden stue 

Lyden af
bøjlen når hun
hængte sin
frakke og 
kom hjem



                  "),
("Vejen",17179870208,"Flamberet 

Hun var 
utroligt intelligent
Man hørte det
på hendes
gakken der 
blev til skræppen når
hun var tændt


30. marts 2004
          "),
("Vejen",68786585600,"For sent

Det er for sent at
tage din hånd  

Det øjeblik der
stod parat med
forventningsfulde
øjne måtte
følge tidens gang og 
blev afløst af et 
andet der ikke er med på 
den slags pjank


17. april 2004"),
("Vejen",274911461392,"Forår

Mens træet sprang ud
gjorde hun orden i
sine skuffer
fjernede sine tåreting
og lukkede sit
hjerte af
Nøglen deponerede
hun hos en
bekendt

Der var ingen planter
at vande


14. februar 2004"),
("Vejen",69860330496,"Forligelse

Du rækker mig
hånden med det
tilhørende
smil

Du siger de
forkromede ord
og spiller bolden over
til  mig

Jeg dribler frem og
tilbage uden mål
fordi jeg ikke kan
finde dit hjerte 


11. februar 2004



                  "),
("Vejen",268468224,"Forlis 

Som en duvende 
ballon 

drivende ind i 
mørket 

forsvinder dagens 
lyseste drøm og 
skimler et 
frostkys på 
tankens forlis 

Nøgne grene 
griber kulden og 
holder fast 


5. marts 2004


                  "),
("Vejen",550025306144,"Forsvar

Det eneste forsvar mod
mørket er 
glæden over
lyset og
tilliden til den der 
gav det magt


13. april 2004"),
("Vejen",77310754816,"Forår

Lærker hængt op over
engen
Små bølger der
sætter lyd på lyset og
fuglestatister i
lange streger mod
himlen der rammer
den forårsblå så
tæt at vi et
øjeblik glemmer
fornuften 
sluger evighed i
store lunser 
slipper jorden og
lader os drive væk som
små dovne skyer der kun
har travlt med at
være til


16. april 2004

 



                  "),
("Vejen",268437504,"Fremad

De vil frem -
ikke sinkes af de
svage
Morgenlyset forgylder
de beslutsommes
færd og kaster 
skyggerne bagud
til mørket 


25. marts 2004"),
("Vejen",618627440909,"Frugt

Barnet på hans
skulder
plukker den
frugt han
ikke kan nå
De ler mellem
kloder og skønt
han ved det bliver 
vinter har
han mistet sin
angst og sin
trang til at ville
forstå


10. april 2004"),
("Vejen",17179870210,"Gourmet

Han har ikke
opfundet den
dybe tallerken men
bruger den
virtuost


22.marts 2004

                  "),
("Vejen",17179869184,"Grænse

Udenfor flyver flagene i
vinden og slår smæld over
deres indgang og udgang
Styrede ansigters
øjne filtrerer
indtryk 
skaber afstand
holder fast
Gennem hylden med
kager i cellofan
flytter han sin
grænse ud bag et
passende offer 


16. marts 2004


          "),
("Vejen",512,"Gråd

Han græder-
altså piller han 
løg 

Hans båd driver over
den ugrædte sø i
bjergets mørkeste
hule


4. februar 2004
                  "),
("Vejen",131072,"Gul

Uophørligt strømmer
regnen ned over
gaden der
forsvarer sig med
løsslupne lysglimt og
svømmende spejle
Grå silhuetter ser
ingenting 
sammenfoldede som 
knive på
jagt efter tørre
lommer
Løsrevet en gul
kjole under en
opspærret mund der
sluger den 
himmelske saft



27. marts 2004"),
("Vejen",25165824,"Gummi

Huset har mure af  
gummi og 
skillevægge af stål
Det er svært når
det blæser eller når
nogen skrider over
grænsen for hvor 
langt man kan gå


26. marts 2004
          "),
("Vejen",16818176,"Hallooo

Mon skaberen har lommer i
tøjet han går og gemmer
ting og sager i?

Jeg er bange for jeg er 
røget i en gammel 
frakke han har glemt han 
har puttet noget i


24.februar 2004
                  "),
("Vejen",0,"Haps

Afmægtig bed han sig
fast i tiden og 
forsøgte at hænge på
men slap med 
livsangst i
munden 
gav op og fangede en 
sommerfugl i
flugt mod det
åbne hav


29. marts 2004"),
("Vejen",2147483648,"Helliget

I den potente morgen
hører jeg mågerne
skrige dagen ind over 
de dorske huse
Jeg hører lågene flyve af
æskerne i det gyldne
lys  og da  gardinerne glider til 
side for livet tændes mit
blik af de skarpe brande på
fuglefjeldenes altaner

Helliget afventer
verden vores berøring


27. februar 2004






                  "),
("Vejen",16777216,"Henstand

Jeg ved  godt det er 
løgn men alligevel 
giver jeg mig hen til
den sarteste drøm
og lader mig løfte
over sidste 
betalingsdato der
må finde mig hvis
den kan


2. marts 2004
          "),
("Vejen",8224,"Hoppet

Vi hopper gennem
dagen på
sten der kan 
bære og
fortsætter gennem
livet til vi
falder ned i
evigheden og
forener os med
det vi troede vi 
var hoppet fra


24. februar 2004
                  "),
("Vejen",8594128928,"Hvaler

Han gik langs havet på
kanten af  gule
klinter og så de 
små hvaler der
legede mellem
skumtoppene
nedenfor
I årene efter fulgte
de hans
hjerte og da 
tiden kom dykkede 
han ind i sin 
erindring og 
fulgte dem ud 
mod det åbne hav


23. marts 2004


                  "),
("Vejen",206158462976,"I kærlighed

Hun kender ikke
resultatet men
man taler til
hende som til et
barn der ikke skal
med


18. februar 2004
          "),
("Vejen",69793251330,"Indsats

Der kan komme et
øjeblik hvor man
har lyst til at
lukke øjnene og
lade verden falde og 
somme tider er det
netop det der 
skal til


14. april 2004"),
("Vejen",16777232,"Kløver

Som forladt af 
Gud 

af
mennesker

bøjer han sig og 
plukker en
kløver

Sådan var det altså for
mig tænker han og 
ser sig omkring


1. marts 2004


                  "),
("Vejen",8606711808,"Klip

Nogen har 
klippet ham fri af hans
baggrund og limet ham
fast til en anden der
ikke har tænkt sig at
tage imod
Et sted er der hul men
her er han bare et
lag uden på


16. februar 2004
                  "),
("Vejen",16778240,"Konversation

Hun blev angst for
stilheden omkring
sig og skruede op
for sin talestrøm
som var den en
bro der kunne
føre hende over 
selskabets 
resignation


31. marts 2004
          "),
("Vejen",50331648,"Løsladt

Uvarslet opstod et
hul i tiden og 
han blev fanget ved et
fodgængerfelt og
hængt på knage i
intetheden der havde et
andet perspektiv end han

På ryggen af et
sekund fløj han
fri af sine sanser og da
dagen flød ind i
timen så han 
medmennesker ved det
grønne lys og sammen
gik de over gaden

Han vidste de var
uden lænker 


9. marts 2004



          "),
("Vejen",318775296,"Meteor

Vi mødes i
tiden og  
brænder op i 
bratte flænger i 
mørket


3. marts 2004
                  "),
("Vejen",2432729600,"Morgen

Mørket indeni
har ingen respekt for
fuglesang


12. april 2004"),
("Vejen",2147483648,"Morgenben

På kanten af dagen
balancerer jeg natten på 
plads i dens
rum
På morgenben snubler jeg over 
månen der har
sat sig fast  mellem 
hemmelige drømme og
en lille pose forbuden 
frugt


4. april 2004"),
("Vejen",2147516544,"Morgenfrugt

Som modne
frugter hænger de
parat til at
lade sig plukke
En bitter erkendelse
over en svulmende
tvivl
En håndfuld glæde
ved siden af en 
fuldvoksen vrede
En stor skam og
allerøverst en tynd
sorg der fanger de
første glimt af sol


13. februar 2004


                  "),
("Vejen",2147483648,"Morgensølv

Morgenens sølvskære
overtræk glider på
plads
Dybrød hæves
solen bag
byens tårne
Natten fejes
sammen i
bunker og
køres væk
Om lidt slippes
lydene løs på den
bløde stilhed


19. marts 2004


                  "),
("Vejen",8192,"Mosaik

Vi aflæser spor i
virkeligheden
afkoder information og
gør den tilgængelig for
betragteren der
tilrettelægger sandheden og
lægger den på
plads i mosaikken


5. april 2004"),
("Vejen",549755813888,"Nødpost

Hjertet banker på
og morser et
budskab jeg ikke
forstår 
Alligevel fatter jeg
min skrøbelighed og
svinder ind til en
bøn der flakser
rundt uden sans for
adressen der er skrevet
godt gemt med 
småt et sted hvor 
ingen plejer at 
kigge


4. marts 2004


                  "),
("Vejen",16777216,"Nar

Jeg vil ikke men
har gjort det
mellem to
hjerteslag

Jeg vil ikke eller
er det mit hjerte jeg
holder for nar


11. marts 2004"),
("Vejen",16777218,"Og dog

er øjnene anbragt så
de ser væk fra en selv


8. februar 2004"),
("Vejen",50332160,"Online

Han bærer stemmerne med
sig som værn mod
den ene
han ikke vil 
høre

Han taster nummeret på
livet for ikke at gå
fortabt


23. februar 2004"),
("Vejen",0,"Ord

Som  snefnug svæver
mine ord gennem din 
stilhed og lægger 
tæppe over sommerens
udtømte kraft


17. februar 2004
          "),
("Vejen",550024257536,"Over

Ikke kroppen eller
sindet
Ikke orkestret eller
partituret
Ikke lærredet eller
farverne
Livet er
og et øjeblik
griber det
materien og
løfter den ind
i lyset


11. april 2004


                  "),
("Vejen",32768,"Regnskab

Der er noget på
færde med dagen i
dag
Jeg mærker den 
har en 
hensigt
Stemningen er i
eksamenstøj og
venter foran
en stålsat dør der 
pønser på sit
spørgsmål


9. februar 2004


          "),
("Vejen",85916126208,"Selvportræt

Hun har et
billede af sig selv som
en kvinde man ikke
siger imod
Rammen er helt
ude af proportion men
det fortæller man ikke en 
stor kanon
                  "),
("Vejen",549755822080,"Sidste afleveringsdato

Når man skal
levere livet
tilbage er
det spændende om
der stilles nogen
spørgsmål
Forhåbentlig er
der ikke tale om
erstatningsansvar for
slid og mangler for
der er vel ligesom
ikke noget at
betale med



23. april 2004"),
("Vejen",68736262144,"Skøjteløb

Vi skøjter  over
papirtynd tilfældighed på 
kanten af et sekund 

i troen på at vore
kræfter skiller os fra 
dem der faldt

Vi genkender ikke
de fremstrakte håb der
kanter vejen frem
men 

holder kraften
i  den tætte
balance  vores
sikkerhed har
skabt


6. februar 2004


                  "),
("Vejen",77309411328,"Skyggen

Med blinde øjne
og døve ører
varmer vi os hos
hinanden til 
skyggen af
løgnen bliver så
indædt kold at den 
lammer vores 
længsel


8. april 2004

                  "),
("Vejen",16777216,"Stil

Jeg  har taget min
glæde på men
kan godt se den ikke
passer så jeg 
hænger den væk igen
og finder min
skuffelse frem som
altid er pæn og
står godt til
hvad som helst dagen
har på  -
og så kræver den 
ingen grimasser


24. marts 2004



          "),
("Vejen",0,"Stilhed

Stilheden den indvendige og den
udvendige der ikke nødvendigvis følges ad men brydes i et felt mellem bevidsthed og drøm
Stilheden mellem støvdragerne på en udsprunget blomst der gløder rummet mellem virkelighed og fantasi
Stilheden mellem to bebyrdede med følelser der er faldet til ro eller ikke vil tillade livet at strømme frit
Stilheden der lægger sig tæt om en død som en kokon der er vævet af liv der er forbi 
Fravær af latente muligheder ventende på at fuldbyrde deres tid
Stilhed emballeret i støj
En tankes magtfulde evne til liv hvor det ikke var forudset


21. marts 2004


          "),
("Vejen",12885950464,"Stjernesæd

Tænk om man kunne
plukke stjernerne og
stø¸ dem ud som
perler hvor man
gik
Gå en tur på
mælkevejen i det
bløde lys fra universets
sære spørgsmålstegn mens 
man stadig havde jorden
under sig som 
rugemor for
himlens sæd


1. april 2004


                  "),
("Vejen",68786585600,"Sult

Jeg sulter efter 
at være dit
daglige brød
men frygter jeg 
ikke er nok


25. februar 2004
                  "),
("Vejen",2048,"Tabt

Det forbudt land
ligger øde
hen

End ikke en
tanke strejfer dets
flader

Og floden der
bærer den
tungeste last
fremviser intet
spor


Grænsen bevogtes
af flammende
håb


20. marts 2004



          "),
("Vejen",137438953472,"Tabt

Han blev afstand og
lod sine øjne være
spejle før han 
hældte sig i
solen

Men skyggen af
hans skygge er her
endnu


7. februar 2004
          "),
("Vejen",274878955520,"Træet

De hænger parat på
grenene 
og lader sig
bruge

falder de af
kommer de vel
igen 

så længe
kraften kan
skabe et 
forår 

af et
piblende
vinterhjerte



12. februar 2004"),
("Vejen",0,"Trafikfilm

Daglige bekymringer
lægger sig som
trafikfilm på
forruden og 
slører mit
udsyn
Det kommer der af at
se verden bag glas 
på sikker afstand af
frodig regn og 
foretagsomt 
levende 
pløre


14. marts 2004


          "),
("Vejen",618542399490,"Uafvænnelig

Hvad bryder
kærlighed sig om
paver


18. april 2004"),
("Vejen",8589937664,"Udvalgsmødet

Han var der med
et halvt hjerte men 
havde rutineret garderet sig med
en skjult dagsorden og
et par knive i ærmerne
Selerne og livremmen 
sikrede ham mod at 
blive taget med 
bukserne nede og kun
for et syns skyld smagte han på 
de rævekager der blev budt om af
mennesker med øjne i
nakken



15. februar 2004"),
("Vejen",0,"Under isen

Øjnene under isen
ser og ser men
lader intet trænge igennem 
til uudgrundeligheden
Dybt under fladen
vokser en rød tulipan
af den dyndede bund
i ly af smertens blade
Et strejf af den
fjerne sol tegner 
gyldne streger i det
strenge mørke 


29. februar 2004


          "),
("Vejen",33554432,"Vagten

Han stivner alle
udtryk
Siger kun 
nødvendige ord
på vagt ved den 
knyttede by
Mellem hans bryn
løber en  fure  som
en sprække i
såret træ


22. april 2004"),
("Vejen",8589934592,"Vejen

Landet udstrakt i 
havet
Himlen og vejen
der løber sammen 
et sted i vores
længsel efter
kontinuitet
Lavthængende
skyers balance ad
nødvendige baner
skriver dagsorden for
dage vi kun kan
tage imod


12. januar 2004

                  "),
("Vejen",77376528384,"Vejen hjem

Med hav på
begge sider går vi
lige ind i
himlen
Mellem drivende
skyer finder vi
vejen hjem og
på randen af et 
terneskrig forstår vi
at vi nær var gået
fejl af livet og
hinanden


20. februar 2004
                  "),
("Vejen",0,"Velsignelse

Ingen græder over
kronbladenes fald mens
frugten vokser sig
nøgen for
skjul


15. april 2004"),
("Vejen",77309411328,"Verden

Nogle blev
jorden jeg står på
Andre udgør min 
baggrund mens en flok
fløj over mit hoved
og synger fra det 
højeste træ i
haven


26. februar 2004"),
("Vejen",206275903488,"Vi

Hun sætter sig
ved siden af
ham og klistrer
hans billede over
det ansigt hun ikke
vil se
Hun imiterer hans
stemme og 
overspiller smertens
lyd mens hun
tager hans hånd og
lader den stryge 
sin


7. april 2004


                  "),
("Yrk",69793219584,"Åndelig udvikling

Enhver plante vil
forsøge at indtage den
plads den har behov
for
Vi mennesker er
højere udviklet så
derfor ser man  ikke på
vort niveau individer 
der presses af 
artsfæller
          "),
("Yrk",0,"2002-2003
          "),
("Yrk",10737451008,"Aflysning

Intet fungerer i dag
beklager
Knæene aflyser
samarbejdet med
kroppen der kun
vil ligge på grund af
uoverkommelighed
Maven protesterer
tungt over vanskelige
forhold mens hjernen
er ramt af morgenchok 
Der er i det hele 
taget ikke noget at 
komme efter
Dagen får intet at
gøre her  
                  "),
("Yrk",17179869216,"At drages med døden

Hun havde lagt sig
med pilleglasset

Nu skulle døden have
det sidste ord

Indkroget i sit eget
mørke mærkede hun
forvirret at hun -
skulle - og 
efter bitter kamp med
kroppen måtte hun
stå op og 
overgive sig til
dens krav

Og da hun så
var oppe kunne hun jo
ligeså godt tage
livet med


10. februar 2004


                  "),
("Yrk",0,"At gøre myg til elefanter 


Nu skal man jo 
ikke gøre myg til 
elefanter 
men hvis det nu 
skete alligevel 
at myg blot blev på 
størrelse med 
elefanter men ellers 
blev ved med at være dem 
de var så kunne det jo 
godt være der var et 
synspunkt eller to der 
skulle revideres og en 
lov eller måske flere der 
skulle justeres for at 
klare problemerne med 
de store dyr der vel fik 
behov for en stærkt forøget 
mængde blod ligesom man 
må antage at myggestik 
ville udgøre en ny trussel for 
de stakler der skulle lægge 
krop til 
Myggesværme i 
sommernatten ville 
blive et nyt eksotisk 
indslag i naturen der nok 
ville kunne få en til at spærre 
øjnene op for ikke at 
tale om ørerne der skulle 
klare en støj der utvivlsomt 
ville være overdøvende 
Vi ville for første gang 
opleve at kunne se en 
myg i øjnene og få et 
helt nyt indblik i de uhyre 
skabningers sjæleliv 
At klaske en myg og tvære den 
flad ville nok være en 
uddøende sport der kun 
kunne udøves af 
specialister 
Det er i grunden ikke meget der 
skal til for at ændre ens liv 
ret voldsomt 
For eksempel bare det at 
gøre myg til 
elefanter 


          "),
("Yrk",1091567872,"Babymagt

Det er i de få
øjeblikke vi har 
held til at 
distancere den indre 
baby vi formår at 
spille voksne –
men i det lange løb er det
babyen der vinder og
når alt kommer til alt er
det vel derfor der
stadig findes rester af
livsrum i verden


          "),
("Yrk",3072,"Besynderligt

Ja hun har da
godt nok en stor
næsen -
men at den i 
den grad skulle tage 
udsynet fra hende 
er ikke til at
forstå


          "),
("Yrk",68719476738,"Blufærdig

Sandheden er for
blufærdig til at
optræde nøgen
hvorfor man oftest
ser den iklædt et
passende kostume
                  "),
("Yrk",68719477760,"Brolæggeren

Hun talte så
højt og så
længe at begge
mine ører
faldt af
Først bagefter
så jeg den 
brolagte vej der
voksede i hendes
spor
          "),
("Yrk",69793220610,"Budgetforlig

Når den sidste
frikadelle er
ædt er det
for sent at
lange til
fadet

                  "),
("Yrk",68719479808,"De siger


Vil du angribe direkte
undgår du ikke
at forholde dig til
sandhedens spejl
Et udspredt rygte
ser ingen i
øjnene og gør
sit arbejde uden
kamp
          "),
("Yrk",32768,"Den omvendte

Hvor er det
ærgerligt jeg har 
brugt så meget 
tid på at iagttage og
kritisere det, det er
så sjovt at gøre
                  "),
("Yrk",0,"Den søgende

Han søgte
originalitet
men fandt den
aldrig da
han ikke kendte
den præcise
definition

24. februar 2004
          "),
("Yrk",18270391296,"Den svævende mand

Ved at hæve sig op
over alt der krævede
holdning undgik han 
svævende at vælge side og 
vandt almindelig beundring
som en meget klog mand


                  "),
("Yrk",17196647424,"Divaen

Hun var altid
på -
selv bag den
låste dør på
toilettet

10. februar 2004
          "),
("Yrk",69809995776,"Drømmefangeren


Hvor der er en vilje
er der en vej
og lige børn leger bedst
Man ved da
den korteste vej til målet
er at håbe lidt
og gøre meget
Husk du er stærkest alene
de små potter
har også ører
og som herre så svend
Sæt tæring efter næring
Tidligt op
og tidligt i seng
det er sundt for en lille dreng
for slet ikke at sige
en sød
lille pige
Som du sår
skal du høste
så den kommer også frem
som ager med stude
Sæt ølse for
og ølse bag
pølsen har dog
samme smag"),
("Yrk",207257404416,"Drivhus

En familie kan
være det
drivhus der
gør det muligt at
overleve en
kold vinter

-men uden en
kyndig gartner
er der nok flere
chancer udenfor
          "),
("Yrk",0,"Efter festen

Den har indtaget
universet og
korsfæstet kroppen
til madrassen

vender han
hovedet
farer den op
og vil ud
                  "),
("Yrk",154618822660,"Eftermæle

De kaldte ham
et fæ så
det var han
jo nok
Hans største
talent var
hans kærlighed
til mennesker
          "),
("Yrk",68720558080,"Efterspil

Alligevel er der
nok tilbage til en
lille fest
En portion glæde
Lidt fremtidstro
Det går sgu nok
Der er en hel flaske
kærlighed "),
("Yrk",618508847104,"En udstrakt hånd

Så enkelt at
markere sin
uafhængighed
Så naturligt at
forsvare en
truet frihed
Så let at rejse
palisader

Så umuligt at
række hånden frem
til forsoning

          "),
("Yrk",69818417216,"Endelig

Jeg har sat filter  i
min telefon
der spærrer for 
opkald fra dem der vil 
skrive min dagsorden
Min brevsprække er
programmeret til at
sønderrive 
henvendelser fra
folk der vil 
forbruge mit liv
Og allerede havelågen
nægter adgang for enhver
der forsøger at gå ind
uden vilje til at dele
min virkelighed et
øjeblik
Endelig er jeg
gammel nok til
at plukke livet 
selv med egne
hænder
          "),
("Yrk",206158430208,"Erindring

Efter hans pludselige
død på stien langs
søen huskede hun at
han godt nok havde
sagt engang at han
ville gå med hende til
verdens ende
          "),
("Yrk",69793285120,"Et eventyr om udeengle og
hjemmedjævle

Efter års ensomhed mødte
han sit alter ego i 
en skøn kvindes 
krop

Når engle mødes, går det
stærkt

De flyttede sammen
i den første nat og 
levede i Helvede til 
deres dages ende


                  "),
("Yrk",1024,"Et spørgsmål om pris

Det var jo det med
kærligheden
Han troede det var når
nogen viste ham
opmærksomhed 
Ja i grunden var han til
salg for et første
vurderende blik


24. februar 2004
                  "),
("Yrk",549755813920,"Fokus

Han fik øje på
døden og 
glemte livet der dog
leveres pakket i
evighedsgavepapir
                  "),
("Yrk",32768,"For højt oppe

Halvvejs til
månen vendte
jeg om  og
skiftede til en
retvendt stråle der
næsten for 
nænsomt fik mig 
hjem til den 
bofaste dag der 
uden at tøve
åbnede favnen for
flaksende øjne og
glødende ører

          "),
("Yrk",69793253378,"For sent

Når den hårfine
revne har åbnet
munden til et
brølende gab er
det for sent at
glatte ud
                  "),
("Yrk",0,"Forførelse

Ønskerne svirrer lavt over
mulighederne og 
lokker dem til 
aktivitet
          "),
("Yrk",35840,"Forræder

Der er ikke
ringeste tvivl i
mit sind
den satan skal
få når jeg ser
ham 
Da jeg står foran
ham er tungen
lam og munden-
min Judas -
den smiler
          "),
("Yrk",86973186048,"Fri


Den frie fugl fløj
højt over
servitutter og panthæftelser
med et skadefro skrig
Grundigt havde den fralagt sig
sine ansvar
og sikret sin frihed
med skyldæg i alles reder
Beboerne ville
med tiden udruge
små byrder, hæftelser og
forpligtelser
der på ingen tid ville
vokse til og for altid
stække deres kraft"),
("Yrk",85899444224,"Frisuren


Da hun omsider
mødte forklaringen på
mislydene i familien
afslog hun
resolut indsigten der
spolerede hendes
syn på nære
personer

Sammenbidt
reparerede hun
frisurens opsætning med
hårde strøg

          "),
("Yrk",627082004480,"Fyldt

Han var så
fuld af sig selv at
der ikke var
plads til andet

          "),
("Yrk",68736319744,"Fædrenes synder

Hendes baggrund
var så broget at
det var svært at
få øje på hende

                  "),
("Yrk",69809997824,"Fællesskabet


Vi er fælles om det meste
Deler alt med hinanden
Lever under samme kår

De forskelle der er
er i grunde ikke forskelle
men nuancer
som farven på vores hår

Vi er i samme båd
det ved vi
og derfor kæmper vi hårdt om 
pladsen først i 
køen
for fordi vi er fælles om det meste
er det ikke alle
som står
bagest i køen
der får"),
("Yrk",549772599334,"Genbesætning

Vi er her jo
side om side en
stund indtil
andre overtager
vore pladser og
afspiller deres 
version
                  "),
("Yrk",66560,"Håb forude

Lider du under 
vægten af en 
alt for stor
familie -
Så vent til du
har været part i den 
første arvesag

          "),
("Yrk",16777216,"Hør her - Makker

Når man møder
en fremmed

kan man til tider
ane et billede af
sig selv

Jeg kan ikke fordrage
det jeg fornemmer
når du sidder og 
kigger på mig
          "),
("Yrk",549755848706,"Himmelfart

At flyve til
himlen på
rosa vinger er
let nok -
Det er 
landingen på
jorden der
er problemet
                  "),
("Yrk",17179902976,"Hjemsøgt

Der var de sygdomme han
ikke havde hørt om og
dem der ikke havde
hørt om ham

- men ellers var han
hårdt ramt
          "),
("Yrk",68719509504,"Hundred og en

Livet kan beslutte at
sætte dig over for 
valget mellem
hundred procent kærlighed 
og
hundred procent kærlighed

Smerten kan lære dig at
der findes kærlighed med
en styrke på hundred og en "),
("Yrk",68719510528,"Husråd

En god hovedpude 
er stoppet med 
andres skyld
          "),
("Yrk",206158430208,"Hvaler

Det siges at
hvaler kommunikerer
over hundredevis af
kilometer

Hvorfor hører du
ikke min smerte

Jeg er enoghalvfjerds
centimeter væk
          "),
("Yrk",33280,"Hvor?

Som barn var
hun bange for at
blive væk fra
sin mor
Senere i livet lærte hun 
at det er værre at 
blive væk fra 
sig selv og ikke
vide hvor man blev af
selvom man sidder 
derhjemme
                  "),
("Yrk",17179901952,"Hysterisk

Er der da ikke nogen der
kan lukke kæften på den 
hysteriske køter der står og 
skaber sig dagen lang 
Den må jo være syg i 
hovedet af at være
lukket inde bag
ligusterhække og palisader mens
det eneste den kan tænke  på er
frække damer og fri adgang til at
lette ben men det er altså 
forbandet at høre på når 
man går her hele dagen og
har nok at gøre med 
at holde det hele pænt


          "),
("Yrk",69793219584,"I al anstændighed

Jeg ved jeg
med tiden vil 
begå et 
overgreb mod
dig men
først vil jeg
arbejde på at
fremstille dig i 
et lys der
får alle til at
indse at jeg ikke
havde andet
valg 
          "),
("Yrk",85899345920,"I al beskedenhed

Så er det jeg har 
tænkt over om 
det er sådan at 
der er nogen der 
har noget imod at 
jeg sidder foran når 
vi kører derover men 
hvis der er nogen der 
gerne  vil sidde der så  
sidder jeg ligeså gerne 
bagi det har jeg ikke 
spor imod det 
er bare sådan at 
jeg let bliver 
køresyg hvis jeg 
ikke sidder foran men 
I må endelig sige til hvis 
I hellere vil sidde foran for 
jeg klarer det sagtens bagi jeg 
kigger bare langt ud ligesom 
når jeg er ude og sejle så 
mærker jeg ikke det 
mindste og man sidder jo 
godt beskyttet bagi 
men nu siger I altså til jeg 
vil ikke være til 
besvær for nogen


          "),
("Yrk",2048,"Integration

Han var godt nok
sort men så 
optaget af at
tilpasse sig de 
hvide at nogle tog 
fejl og 
forvekslede ham med 
en programmeret
albino
          "),
("Yrk",17196647424,"Ja hvem ved

Som ung blev han
spået at han ville
ende sit liv som
en stor vismand
Livet igennem blev
han behandlet for
en psykisk lidelse der
gav ham en fremtræden 
som en skæv kopi af 
paven parret  med 
Dalai Lama


                  "),
("Yrk",68736286720,"Ja ja da

Hun efterstræbte ham
utrætteligt til
han gav op og
lod sig blive hendes
hustyran


11.januar 2004
          "),
("Yrk",550829555712,"Kaffemik

Verdensordenen
lægger sig tungt om 
min hals og 
forlanger lyd
Verdensfreden og
den globale opvarmning
står parat med 
grundvandets forgiftning og 
kræver svar mens det 
evige liv
hvisler ondt fra
kulissen
Er der noget
nogen har glemt?
Jeg ser mig omkring
efter hjælp  fra 
familiens problemer 
for slet ikke at tale om 
økonomien der som altid
stikker hovedet i 
busken og lader som 
ingenting
Jeg smider min glæde over 
livet på bordet men indser
det er ingenting mod 
al landsens ulykker der 
sidder til bords med 
en rødmende skam over
et livs resultater der 
tier og tæller på 
en hånd mellem 
tilværelsens onde tanter der 
ikke giver op før 
tallerkenen er 
tom


          "),
("Yrk",32770,"Karakterbrist

Stilheden har det
med at stille
ubehagelige
spørgsmål
                  "),
("Yrk",94506157056,"Kludetæppet

Hun lignede sin
farmor til 
forveksling
havde sin haveinteresse fra 
sin far
sine kulinariske evner fra sin 
mor mens et lidt vel 
fantasifuldt træk kom 
fra hendes oldefar der 
var kendt som en stor fortæller
En fjern tante have skænket hende
appetit på kønne mandfolk mens 
hendes udprægede trang til
kommunikation mentes at 
udspringe fra en oldemor på 
moderens side
Således begavet slog hun sig ned i 
sofaen med en kop te hyllet i 
sit kludetæppe

Hendes trang til 
magelighed stammede fra 
en tante der heller ikke
hørte til de slanke



                  "),
("Yrk",68719509504,"Kommentar


Om du så kvækker hele telefonbogen
af dig
eller udvalgte dele af
Nudansk Ordbog
parret med indholdet af
Fremmedordbogen
sovset rundt i Psykologisk Håndbog
kan du spare dig
Dit intellektuelt
manipulerede budskab er
galende ligegyldigt
og så kunstigt
arrangeret
at mine ører
byder sig skamløst til for
hvem som helst
der hvisker bare ét
purt barnligt ord
der er ægtefødt af hjertet"),
("Yrk",32768,"Kontrollant


At føle tanker eller
at tænke følelser er
farlige discipliner der
har skadet mange
Tanker skal tænkes og
følelser føles men det er 
obligatorisk for
begge at være den
andens kontrollant
          "),
("Yrk",549772623872,"Kursændring?

Du
mit vidne
- hvis du er der
et sted 
Du ved jeg 
har stræbt efter
kærlighed 
Se hvordan 
det gik 
Måske skulle jeg
prøve at
hade lidt


          "),
("Yrk",69256347652,"Livsløbet

Midt på ruten sprang
kæden af og han
måtte gå resten af 
vejen hjem mens
de øvrige deltagere
en efter en kørte
forbi med glade
smil


21-12-2003


                  "),
("Yrk",68719476736,"Lydløs

Så let som
ingenting fløj
sætningen af sted og
plantede en sten i
hjertet på én der
havde håbet på en 
god dag

Den blev aldrig
udtalt



          "),
("Yrk",68719477760,"Lykken den er…

Han var godt
gift -
cyklede altid
bagest i 
modvind
          "),
("Yrk",549755813890,"Lys

Vil du finde lys 
må du afsøge 
mørket
                  "),
("Yrk",68719478784,"Måske

Sædvane holder
møde med
tradition om et
fælles fodslag
mens plejer smiler fra 
væggen

Fremtid er på
prøve
          "),
("Yrk",68736319744,"Medmennesker

Først er vi sønner og
døtre så
bliver vi fædre og
mødre
Måske oplever vi
at blive børnenes
børn til sidst og
undgår helt at
være hinandens
medmennesker

4.september 2003
          "),
("Yrk",85916126208,"Ministerfruen

Han havde giftet sig
med den yndigste
lille spurv der nu 
måtte tage timer i
tranedans

          "),
("Yrk",32768,"Misundelse

Dyr skider
Mennesker har
afføring
Alligevel blev jeg
misundelig da jeg
så den lort
naboens Pølle
plantede på
mit fortov

          "),
("Yrk",68719542272,"Moders hånd

Hun var stærk i
rollen som
mor
Bankede med
lethed både
mand og børn


24. februar 2004
          "),
("Yrk",69793218560,"Modvind

I kampen med vinden der 
altid var imod
opdagede du ikke at 
du for hvert skridt frem 
lagde et skridt til 
afstanden mellem os
          "),
("Yrk",68719543296,"Mor

De havde ingen
børn
Det ville aldrig gå 
Han have selv brug for 
al den omsorg der
var at få


10. februar 2004
          "),
("Yrk",0,"Museum


På museet stod
servicet Marie Antoinette brugte
i sit fangenskab før hun fik hovedet hugget af
Hendes følelser i den anledning
var dog ikke at se og
ikke en tanke undslap
det brugte stel
Kopperne overlevede hele nok
deres hovedløse dronning
skønt hun med hovedet intakt
næppe skænkede dem en tanke
Hvor dronningen er
ved jeg ikke
men porcelænet findes altså
og kan beskues
så ofte nogen løser billet
          "),
("Yrk",566953549824,"Myrer

Da hun indså at der
fra hende selv var
lige langt i alle
retninger satte hun sig
fornøjet i centrum og 
bed en luns af 
livet som hun 
nød mens hun dovent 
betragtede mylderet af 
myrer på
terrassens slidte
fliser


                  "),
("Yrk",0,"Næsten

Jeg var
fuld
i går

Jeg er
ikke fuld
i dag 


Men jeg vil nok
være fuld
i morgen
igen


18. januar 2004"),
("Yrk",549755813888,"Nå sådan

De fleste ved at
livet sluttes af
døden
Alligevel lever
vi som om vi
har indtaget
det evige liv og
det er måske
netop det der er
livets mening

          "),
("Yrk",68719479810,"Nåde

Der findes 
emner man har
lov til ikke at
have en
menig om
- f.eks. dem
man ikke har
indsigt i
                  "),
("Yrk",86973090816,"Notat 

Set ud fra en 
kølig betragtning er der 
ikke meget at 
bemærke - 
men hun har jo altid haft 
et ildfuldt blik 




December 2004
                  "),
("Yrk",68719509504,"Om kærlighed og bytteleg

Hvis jeg var dig
og du var mig
kunne vi elske hinanden som nu
Jeg elsker dig
og du
mig

Elsker du mig, elsker jeg dig

Hvis jeg var dig
som mig i dig
og du var mig
som dig i mig
kunne vi elske os selv
i hinanden

Elsker du mig, elsker du dig
Elsker jeg dig, elsker jeg mig

Du skal elske din næste som dig selv

Det er vel bedst
hvis jeg er mig
og du er dig
og kærligheden
ingen bytteleg

Jeg elsker dig.

Elsker du mig?"),
("Yrk",1090519044,"Omsider

Efter et langt livs
ambition
går man sin
afmægtighed i
møde og byder
den velkommen
          "),
("Yrk",68719476736,"Op og ned

Tæppet gik op for
virkeligheden og
ned for illusionen
                  "),
("Yrk",549772591104,"Opskrifter

Du gør som
anvist og
kommer frem til
et resultat der
til forveksling ligner
hvad du blev
lovet men
indser at du intet
har at gøre her
hvor du ikke
genkender dit eget
landskab der
solsvedent venter din
forløsende regn


          "),
("Yrk",94506124288,"Oserinden

Der var ikke
tale om at hun
på nogen måde
forsøgte at skabe
et forkert indtryk
Nærmest ved et
uheld gled hun
over i en fornem
frues attitude der
næsten på egen
hånd gik i 
realitetsforhandlinger
om en  millionlejlighed
der dog heldigvis
viste sig at  have et
køkken der vendte 
mod nord

Man vender jo
ikke lige en
boligblok




                  "),
("Yrk",17179870208,"Overfaldet

Den flyvende fedtklump
slog til igen da
den ramte mig
klistret midt på panden
drev ned over fyldige æblekinder
og formede yppige hængebryster
for efter en tid at falde til ro
i vuggende hofter
over fremfalden mave
alt båret frem
på latterligt tynde
ben."),
("Yrk",1073743874,"Overflod

Der er stjerner nok
til os alle
så længe vi ikke kan
nå dem"),
("Yrk",17196647424,"Personlighed ?

Det du ser er
et lærred og
en projiceret
film

Kigger du efter 
kan du gennem 
overfladen ane en 
lille person der
helt alene holder 
showet i gang
          "),
("Yrk",636779234304,"Prædikanten

Han forkyndte
Guds vilje som
han forstod
bedre end
menneskets 
virkelighed
          "),
("Yrk",86989868034,"Prinsen

Han var altid
prins
- også om 
natten

                  "),
("Yrk",68719476736,"Rådgiveren

Jeg er aldrig i tvivl om
hvad der bør gøres -
med mindre jeg selv
er involveret


10. februar 2004

          "),
("Yrk",68719476736,"Rationalisering

Ofte tager det
længere tid at
finde en undskyldning for 
ikke at hjælpe end 
det ville have gjort at 
give en hånd med
          "),
("Yrk",618475290624,"Realisten


Bær hinandens
byrder -
men let på 
jeres egen
først
          "),
("Yrk",1073743874,"Redskab

Måske er fortiden det
redskab du skal
bruge når du fra
nutiden forsøger at
påvirke fremtiden
                  "),
("Yrk",568009426944,"Reformatoren

Resolut skar han
igennem til
sandheden og indså at 
han havde misforstået
alt


27-12-2003
          "),
("Yrk",68719509506,"Rekord

Øjne løber
hurtigere end 
ben —
også baglæns
                  "),
("Yrk",3072,"Reportage - med indlagt stilbrud

Majestæten skred frem
ad det røde tæppe mod 
det forventningsspændte
bånd 

Det gyldne klip frosset i
blitzlys under
en konges hånd

Herefter vendte regenten
snuden direkte
hjem

(Denne tekst blev - med et vedlagt smil - sendt til en journalist ved Fyens Stiftstidende efter en reportage om royalt besøg i Odense)





                  "),
("Yrk",68719476736,"Risikovurdering

Det, du selv yder,
er du sikker på.
Det, du forventer dig af
andre, er en
risikabel affære.

          "),
("Yrk",69793220610,"Risikovurdering

Giver du dig i
kast med en
gordisk knude
kan det ende med at
det er dig der
bliver hugget over


                  "),
("Yrk",86989864960,"Rygrad

Da hun fandt en 
hun kunne se op til
klatrede  hun 
behændigt op og 
sprang ud i 
røde roser

          "),
("Yrk",2,"Så længe…

Så længe musikken spiller, kan der danses.

                  "),
("Yrk",69793284096,"Sølvbrylluppet

Jo vist vandt du din
magtkamp men du
forstod aldrig at
dermed tabte vi alt



          "),
("Yrk",68719476738,"Saltvand

Det er vel med
kærligheden 
som med
grundvandet
Hver gang du
overøser jorden
med salt stiger
det bitre lag
                  "),
("Yrk",619549032448,"Samarbejde

At den  Almægtiges
kærlighed er
adresseret til alle
er et tegn på
hans uansvarlighed

Det ved vi der
har rettet op og
defineret vores
forskellighed

          "),
("Yrk",1073743872,"Samarbejde

Saglige begrundelser 
opmarcheret på 
to rækker
gør holdt foran
den folkelige opinion der
længe har forsøgt at
finde et ståsted
På tværs af flugtvejen har
man anbragt en 
folketingsmand der uden 
at blinke kan åbne for
helvedes porte og 
anvise vej til
Slaraffenland

          "),
("Yrk",69793221632,"Sammenhold

Skumfødt hilser du
lyset mens du krydser
sumpen stående på
ryggen  af  den du
har trådt ned i 
sølet
          "),
("Yrk",17179870208,"Sammensat

Han var ikke så
høj og ligesom sat
sammen af to dele der 
ikke passede til
hinanden
Det var udelukkende
den store der
førte ordet og man
kunne ikke lade
være med at
spekulere på hvad der
var blevet af resten af 
den lille der måtte
nøjes med at
være ben


November 2004


          "),
("Yrk",69793218560,"Samspil

Vil kroppens organer
være singler 
kollapser 
universet



          "),
("Yrk",85899412480,"Sardin

Som en sardin i
sin dåse trækker hun i
egen saft mens hun
smager bitterheden til
sidste dråbe

          "),
("Yrk",18253614080,"Selvhjulpne

Selvhjulpne begyndelser til et 
forehavende jeg ikke kan 
godkende er allerede så 
godt i gang at jeg for 
et syns skyld må give 
en hånd med
Alt tegner jo i øvrigt til
et heldigt resultat
          "),
("Yrk",68736256002,"Slikmund

Så længe man
kan forklare
alt ubehag som
andres skyld kan
man selv blive 
siddende ved
konfektskålen


                  "),
("Yrk",566952460288,"Snøren

Han havde levet sit
liv i et langt
fremadrettet kast
Da det blev tid at
trække linen ind
kom han i tvivl om
der var bid
          "),
("Yrk",549755813888,"Snusfornuft

Herregud
Er der noget at
være bange for?
Hvor var du et 
år før du blev
født?
          "),
("Yrk",85899346944,"Spekulation

Hun ville
utvivlsomt have været
til inspiration for
Mother Theresa -
dersom denne havde 
kendt til hendes
eksistens
Til gengæld ville nonnen vel 
have taget vare på 
den gæve kvindes børn og
hendes overtrætte mand
          "),
("Yrk",619565811712,"Stregen

Der er dem
på den ene
side og dem på 
den anden
Dem med den
rigtige farve og dem
med den forkerte
Den rigtige mening ved altid
hvilken side den er på
som ret og uret der
mirakuløst af sig selv
finder hjem
Hvordan man kan
være på den forkerte
side er mig en gåde
der undgås når
man kaster et 
blik over
stregen


          "),
("Yrk",85899412480,"Svigerinden 1

Hvad sagde jeg

Sagde jeg det ikke nok
og nu har vi skandalen
enhver kunne have
forudset at de ikke
havde råd
Og den bil ja
sådan noget koster
men de ligger som de
reder det er nu min
mening og har du så 
hørt at de to hankaniner
har fået unger et rod over 
det hele du behøver bare 
kigge på køkkengulvet men 
ikke mine ord igen


          "),
("Yrk",25769870336,"Svigerinden 2

Menuen

Og vi skal have 
mørbrad fyldt 
mørbrad med
persille den 
skønneste persille jeg 
selv har fyldt i en 
lækker mørbrad og 
pillekartofler små 
pillede aspargeskartofler med 
drys og ved du 
hvad asier rigtig 
hjemmesyltede asier som jeg
har købt på torvet og
syltet en hel lørdag det 
bliver  lækkert og så skal 
vi have trifli vi 
skal have trifli til 
dessert kanontrifli med
sherry en dejlig 
kanontrifli med
sherry fyldt mørbrad 
fyldt mørbrad og kanontrifli
en rigtig skøn trifli
og kartofler med drys



                  "),
("Yrk",0,"Så menneskeligt

Rolig frue
rolig
Vi er ikke skabt som
engle - husk:
de har ingen
maver
                  "),
("Yrk",32770,"Tømmermænd


Efter en lang aftens snak
sludren og tæt druk
er de tømmermænd der
kommer træskohamrende
på grund af spritten
de elskværdigste
                  "),
("Yrk",86989864960,"Tarzan

Jeg må kaste mig ud af 
stilheden og forsøge at 
sætte foden på
et par nakker 
Tortere luften med 
hunkønshvin
Brøle mig larmende gennem
junglen og vælte et par 
småbørn inden jeg igen kan 
lade mig betjene af
alt det jeg må lade 
ligge her
          "),
("Yrk",69793218560,"Thi kendes for ret

Hvis man absolut vil være
sine omgivelsers dommer
må man vel anstændigvis
opgive også at være
anklager og forsvarer
          "),
("Yrk",618475291648,"Tilgivelse


Alle
tilgiver jeg

Alt

Kun ikke dig
jeg forbrød mig
imod"),
("Yrk",0,"Tja…

Lige så umuligt at
gendanne sin
fortid som
håbløst at
konservere sin
nutid

Fortid skal vel
forstås
Nutid leves og
fremtid tages 
imod

          "),
("Yrk",32770,"Transport

De tungeste
byrder bæres i
hjertet

                  "),
("Yrk",10246,"Trin

Vi går op ad
trapper andre
har opført
for at lægge
vore trin der
en dag bringer
nogen til en
ny etage


                  "),
("Yrk",2147516418,"Tung

Det var en af de dage
der var gået i stå så
man selv måtte slide 
ved hejseværket

                  "),
("Yrk",0,"Ulykke

Drømmen glemte at
tage sig i agt for
mulighederne og 
blev slået til
skuffelse
          "),
("Yrk",755914310660,"Ved begravelsen

Familien bliver
lille, sagde hun
hændervridende
men overså at de
endnu levende
behandlede hinanden så
ufølsomt at flere i 
følget skævede
langt efter 
gravens privatliv
          "),
("Yrk",69793219584,"Verdensbilledet

Han stillede
sig i kreds om
sin kone indtil
hun troede
verden var
ham
          "),
("Yrk",2050,"Vurdering 

Det er ufarligt at 
kritisere detaljen 
så længe man ikke 
anfægter systemet 




Januar 2005
                  "),
("Yrk",69793218560,"tête-à-tête

Selvom der ikke
er langt over
bordet er der
isfjelde mellem
vore fikserede
kuverter så
intet budskab
når over de
kolde toppe bag
hvilke solen netop
gik ned


          "),
];

//fn check_oplaesning() {
//    use std::path::Path;
//    for (i, (samling, temaer, digt)) in DIGTE.iter().enumerate() {
//        let tma = if Path::new(format!("MP3/{i}.mp3").as_str()).exists() {
//            temaer | 1 << 33
//        } else {
//            temaer & !(1 << 33)
//        };
//        println!("(\"{samling}\",{tma},\"{digt}\"),");
//    }
//}
