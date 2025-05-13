from sentence_transformers import SentenceTransformer
import json
import time

# 1. 書籍タイトルなどを準備（例として仮データ）
books = [
    {"isbn": "0195153448", "title": "Classical Mythology"},
    {"isbn": "0002005018", "title": "Clara Callan"},
    {"isbn": "0060973129", "title": "Decision in Normandy"},
    {
        "isbn": "0374157065",
        "title": "Flu: The Story of the Great Influenza Pandemic of 1918 and the Search for the Virus That Caused It",
    },
    {"isbn": "0393045218", "title": "The Mummies of Urumchi"},
    {"isbn": "0399135782", "title": "The Kitchen Gods Wife"},
    {
        "isbn": "0425176428",
        "title": "What If?: The Worlds Foremost Military Historians Imagine What Might Have Been",
    },
    {"isbn": "0671870432", "title": "PLEADING GUILTY"},
    {
        "isbn": "0679425608",
        "title": "Under the Black Flag: The Romance and the Reality of Life Among the Pirates",
    },
    {"isbn": "074322678X", "title": "Where Youll Find Me: And Other Stories"},
    {"isbn": "0771074670", "title": "Nights Below Station Street"},
    {
        "isbn": "080652121X",
        "title": "Hitlers Secret Bankers: The Myth of Swiss Neutrality During the Holocaust",
    },
    {"isbn": "0887841740", "title": "The Middle Stories"},
    {"isbn": "1552041778", "title": "Jane Doe"},
    {
        "isbn": "1558746218",
        "title": "A Second Chicken Soup for the Womans Soul (Chicken Soup for the Soul Series)",
    },
    {"isbn": "1567407781", "title": "The Witchfinder (Amos Walker Mystery Series)"},
    {
        "isbn": "1575663937",
        "title": "More Cunning Than Man: A Social History of Rats and Man",
    },
    {"isbn": "1881320189", "title": "Goodbye to the Buttermilk Sky"},
    {"isbn": "0440234743", "title": "The Testament"},
    {"isbn": "0452264464", "title": "Beloved (Plume Contemporary Fiction)"},
    {
        "isbn": "0609804618",
        "title": "Our Dumb Century: The Onion Presents 100 Years of Headlines from Americas Finest News Source",
    },
    {
        "isbn": "1841721522",
        "title": "New Vegetarian: Bold and Beautiful Recipes for Every Occasion",
    },
    {
        "isbn": "1879384493",
        "title": "If Id Known Then What I Know Now: Why Not Learn from the Mistakes of Others? : You Cant Afford to Make Them All Yourself",
    },
    {
        "isbn": "0061076031",
        "title": "Mary-Kate &amp; Ashley Switching Goals (Mary-Kate and Ashley Starring in)",
    },
    {"isbn": "0439095026", "title": "Tell Me This Isnt Happening"},
    {"isbn": "0689821166", "title": "Flood : Mississippi 1927"},
    {"isbn": "0971880107", "title": "Wild Animus"},
    {"isbn": "0345402871", "title": "Airframe"},
    {"isbn": "0345417623", "title": "Timeline"},
    {"isbn": "0684823802", "title": "OUT OF THE SILENT PLANET"},
    {"isbn": "0375759778", "title": "Prague : A Novel"},
    {"isbn": "0425163091", "title": "Chocolate Jesus"},
    {"isbn": "3404921038", "title": "Wie Barney es sieht."},
    {"isbn": "3442353866", "title": "Der Fluch der Kaiserin. Ein Richter- Di- Roman."},
    {"isbn": "3442410665", "title": "Sturmzeit. Roman."},
    {"isbn": "3442446937", "title": "Tage der Unschuld."},
    {"isbn": "0375406328", "title": "Lying Awake"},
    {"isbn": "0446310786", "title": "To Kill a Mockingbird"},
    {"isbn": "0449005615", "title": "Seabiscuit: An American Legend"},
    {"isbn": "0060168013", "title": "Pigs in Heaven"},
    {"isbn": "038078243X", "title": "Miss Zukas and the Ravens Dance"},
    {"isbn": "055321215X", "title": "Pride and Prejudice"},
    {
        "isbn": "067176537X",
        "title": "The Therapeutic Touch: How to Use Your Hands to Help or to Heal",
    },
    {"isbn": "0061099686", "title": "Downtown"},
    {"isbn": "0553582909", "title": "Icebound"},
    {"isbn": "0671888587", "title": "Ill Be Seeing You"},
    {"isbn": "0553582747", "title": "From the Corner of His Eye"},
    {"isbn": "0425182908", "title": "Isle of Dogs"},
    {"isbn": "042518630X", "title": "Purity in Death"},
    {
        "isbn": "0440223571",
        "title": "This Year It Will Be Different: And Other Stories",
    },
    {"isbn": "0812523873", "title": "Proxies"},
    {
        "isbn": "0842342702",
        "title": "Left Behind: A Novel of the Earths Last Days (Left Behind #1)",
    },
    {"isbn": "0440225701", "title": "The Street Lawyer"},
    {"isbn": "0060914068", "title": "Love, Medicine and Miracles"},
    {"isbn": "0156047624", "title": "All the Kings Men"},
    {"isbn": "0245542957", "title": "Pacific Northwest"},
    {"isbn": "0380715899", "title": "A Soldier of the Great War"},
    {"isbn": "0553280333", "title": "Getting Well Again"},
    {"isbn": "0961769947", "title": "Northwest Wines and Wineries"},
    {"isbn": "0964778319", "title": "An Atmosphere of Eternity: Stories of India"},
    {"isbn": "0671623249", "title": "LONESOME DOVE"},
    {"isbn": "0679810307", "title": "Shabanu: Daughter of the Wind (Border Trilogy)"},
    {"isbn": "0679865691", "title": "Haveli (Laurel Leaf Books)"},
    {"isbn": "2070423204", "title": "Lieux dits"},
    {
        "isbn": "0345260317",
        "title": "The Dragons of Eden: Speculations on the Evolution of Human Intelligence",
    },
    {"isbn": "0394743741", "title": "The yawning heights"},
    {"isbn": "042511774X", "title": "Breathing Lessons"},
    {"isbn": "0804106304", "title": "The Joy Luck Club"},
    {"isbn": "1853262404", "title": "Heart of Darkness (Wordsworth Collection)"},
    {"isbn": "0312970242", "title": "The Angel Is Near"},
    {"isbn": "1853260053", "title": "Tess of the DUrbervilles (Wordsworth Classics)"},
]


start = time.time()
# 2. モデル読み込み
model = SentenceTransformer("all-MiniLM-L6-v2")

print("Model loaded in {:.2f} seconds".format(time.time() - start))

start = time.time()
# 3. タイトルをベクトル化
vectors = [
    {"isbn": book["isbn"], "vector": model.encode(book["title"]).tolist()}
    for book in books
]
print("Vectors created in {:.2f} seconds".format(time.time() - start))
print("Number of vectors: ", len(vectors))
print(
    "Average time per vector: {:.2f} seconds".format(
        (time.time() - start) / len(vectors)
    )
)

# 4. 保存
with open("./backend/scripts/book_vectors.json", "w") as f:
    json.dump(vectors, f)
