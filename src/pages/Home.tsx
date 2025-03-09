import { useNavigate } from "react-router-dom";
import { Box, Button, Description, FancyInput, MainText } from "../utils";

export default function Home() {
    let navigate = useNavigate();

    return (
        <section className="flex items-center justify-center h-screen text-center">
            <Box width={800}>
                <MainText text="Wgraj swuj plik gołombeczku" />
                <Description>
                    Kohaniutki, wgraj swuj plik, a my go ogarniemy reszte
                </Description>

                <FancyInput label="Plik" placeholder="Wgraj swuj plik" type="file" onChange={() => {} } />

                <div className="flex justify-center gap-8 mt-5">
                    <Button name="Wgraj plik" onClick={() => alert('nie masz jaj hahahaha')} />
                    <Button name="Jebanie disa" onClick={() => navigate("/nigger1/nigger2/nigger3/sexypage")} />
                </div>
            </Box>
        </section>
    )
}