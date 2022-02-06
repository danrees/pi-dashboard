import Home from "./routes/Home.svelte"
import CalendarList from "./routes/calendarList.svelte"


const routes = {
    "/": Home,
    "/calendarlist": CalendarList,
}

export default routes;